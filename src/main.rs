#![feature(proc_macro_lib)]
#![feature(core_intrinsics)]

extern crate proc_macro;
extern crate serde_json;
extern crate linked_hash_map;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate lazy_static;
extern crate rustfmt;

mod jsonext;

use jsonext::ValueExt;
use quote::{Tokens, Ident, ToTokens};
use std::collections::{HashSet, VecDeque, HashMap};
use std::cell::Cell;
use std::rc::Rc;
use std::fs::File;
use std::str::FromStr;
use std::marker::PhantomData;
use serde_json::Value;
use proc_macro::TokenStream;
use linked_hash_map::LinkedHashMap;

type CreateType = Fn(&str, &Value, &mut Context) -> Rc<Datatype>;

struct Context {
    type_constructors: HashMap<String, Rc<CreateType>>,
    json: Value,
    types: HashMap<String, Rc<Datatype>>
}

impl Context {
    fn register<T: Datatype + Sized + 'static>(&mut self, name: &str) {
        self.type_constructors.insert(name.into(), Rc::new(T::create_from_fields));
    }

    fn create_subtype<'a>(name: String, parent_ty: String, default_type_args: Value, new_args: Vec<(String, String)>) -> Rc<CreateType> {
        // TODO: go through default_type_args, find variables.
        Rc::new(move |_, val, map| {
            // Merge default_type_args and val
            let mut new = default_type_args.clone();
            for (k, v) in new_args.clone() {
                new[&v] = val[&k].clone();
            }
            let obj = Value::Array(vec![Value::String(parent_ty.clone()), new]);
            map.create_datatype(&name, &obj)
        })
    }

    fn create_datatype(&mut self, name: &str, ty: &Value) -> Rc<Datatype> {
        if !self.type_constructors.contains_key(get_type_name(ty)) && !self.types.contains_key(get_type_name(ty))
            && self.json["types"].as_object().unwrap().contains_key(get_type_name(ty)) {
            let parent_type = get_type_name(&self.json["types"][get_type_name(ty)]).to_string();
            let parent_args = get_type_args(&self.json["types"][get_type_name(ty)]).clone();
            // Walk through parent_args, figure out if it has any variables.
            // if it does, it's a permanant and should be created + inserted in
            // self.2.
            let new_args = fold_recursive(&parent_args, |mut acc, k, v| {
                if let Value::String(ref s) = *v {
                    if let Some('$') = s.chars().nth(0) {
                        acc.push((s[1..].into(), k.into()));
                    }
                }
                acc
            }, vec![]);
            if new_args.is_empty() {
                let parent_ty = &self.json["types"][get_type_name(ty)].clone();
                let parent = self.create_datatype(get_type_name(ty), parent_ty);
                self.types.insert(get_type_name(ty).into(), Rc::new(WrapperDatatype::new(parent)));
            } else {
                let f = Self::create_subtype(get_type_name(ty).to_string(), parent_type.into(), parent_args, new_args);
                self.type_constructors.insert(get_type_name(ty).into(), f);
            }
        }

        // TODO: Remove the Todo fallback
        let f = self.type_constructors.get(get_type_name(ty)).map(|f| f.clone());
            //.expect(&format!("The datatype {} for {} does not exist", get_type_name(ty), name)).clone();
        match f {
            Some(f) => f(name, &get_type_args(ty), self),
            None => {
                self.types.get(get_type_name(ty))
                    .map(|ty| ty.clone())
                    .unwrap_or(Todo::create_from_fields(name, &get_type_args(ty), self))
            }
        }
    }
}


fn fold_recursive<T, U>(v: &Value, f: T, default: U) -> U where for<'r> T: Fn(U, &'r str, &'r Value) -> U {
    fn recurse<T, U>(cur_key: String, v: &Value, f: &T, mut cur: U) -> U where for<'r> T: Fn(U, &'r str, &'r Value) -> U {
        match *v {
            Value::Object(ref map) => {
                for (k, v) in map {
                    // TODO: Sanitize k
                    cur = recurse(cur_key.clone() + "/" + k, v, f, cur);
                }
                cur
            },
            Value::Array(ref arr) => {
                for (k, v) in arr.iter().enumerate() {
                    cur = recurse(cur_key.clone() + "/" + &k.to_string(), v, f, cur);
                }
                cur
            },
            ref inner => {
                f(cur, &cur_key, inner)
            }
        }
    }
    recurse(String::new(), v, &f, default)
}

#[derive(Debug)]
struct WrapperDatatype(Rc<Datatype>, Cell<bool>);
impl WrapperDatatype {
    fn new(datatype: Rc<Datatype>) -> WrapperDatatype {
        WrapperDatatype(datatype, Cell::new(false))
    }
}

impl Datatype for WrapperDatatype {
    // Delegate most stuff.
    fn create_from_fields(_: &str, _: &Value, _: &mut Context) -> Rc<Datatype> {
        unimplemented!()
    }

    fn rust_type(&self) -> Ident {
        self.0.rust_type()
    }

    fn generate_read(&self, depth: u64) -> Tokens {
        self.0.generate_read(depth)
    }

    fn generate_types(&self) -> Tokens {
        if !self.1.get() {
            self.1.set(true);
            self.0.generate_types()
        } else {
            quote! {}
        }
    }
}

fn fieldref(fieldref: &str, depth: u64) -> String {
    fieldref.split(".").fold((depth, true, String::new()), |(depth, counting_depth, s), cur| {
        if counting_depth && cur == "super" {
            (depth - 1, true, s)
        } else if s.len() == 0 {
            (depth, false, cur.to_string() + "_" + &depth.to_string())
        } else {
            (depth, false, s + "." + cur)
        }
    }).2
}


/// A bytecount of a structural datatype
#[derive(Debug)]
enum Count {
    /// The structural datatype should read the inner datatype to know its size
    Type(Rc<Datatype>),
    /// The structural datatype is fixed in size
    Fixed(u64),
    /// The structural datatype should use the given JSON Pointer to find
    /// its size
    Ref(String)
}

impl Count {
    fn from_val(val: &Value) -> Count {
        match *val {
            Value::String(ref s) => Count::Ref(s.clone()),
            Value::Number(ref n) => Count::Fixed(n.as_u64().unwrap()),
            _ => panic!()
        }
    }

    fn generate_read(&self, depth: u64) -> Tokens {
        match *self {
            Count::Type(ref datatype) => datatype.generate_read(depth),
            Count::Fixed(size) => quote! { #size },
            Count::Ref(ref field) => {
                let ident = Ident::new(field.clone());
                quote! { #ident }
            }
        }
    }

    fn generate_types(&self) -> Tokens {
        match *self {
            Count::Type(ref datatype) => datatype.generate_types(),
            _ => quote! {}
        }
    }
}

trait Datatype : std::fmt::Debug {
    fn create_from_fields(type_name_prefix: &str, fields: &Value, generators: &mut Context) -> Rc<Datatype> where Self: Sized;
    fn rust_type(&self) -> Ident;
    ///
    /// This is used to generate reads. Should return an expression that reads
    /// from the Read instance `read`.
    ///
    fn generate_read(&self, depth: u64) -> Tokens;
    // TODO: generate_write(&self) -> Tokens;
    ///
    /// Used to generate types like structs/enums necessary that are then
    /// returned when reading a subtype of this datatype.
    ///
    fn generate_types(&self) -> Tokens {
        quote! {}
    }
}

#[derive(Debug)]
struct SignedNumber<T> {
    phantom: PhantomData<T>
}

#[derive(Debug)]
struct UnsignedNumber<T> {
    phantom: PhantomData<T>
}

impl<T: 'static + std::fmt::Debug> Datatype for SignedNumber<T> {
    fn create_from_fields(_: &str, _: &Value, _: &mut Context) -> Rc<Datatype> {
        Rc::new(SignedNumber::<T> {
            phantom: PhantomData
        })
    }

    fn rust_type(&self) -> Ident {
        // type_name is pretty safe. rust-lang/rfcs#1428
        Ident::new(unsafe { ::std::intrinsics::type_name::<T>() })
    }

    fn generate_read(&self, _: u64) -> Tokens {
        let size = std::mem::size_of::<T>();
        let ty = self.rust_type();
        quote! { byteorder::ReadBytesExt::read_int::<byteorder::BigEndian>(read, #size)? as #ty }
    }
}

impl<T: 'static + std::fmt::Debug> Datatype for UnsignedNumber<T> {
    fn create_from_fields(_: &str, _: &Value, _: &mut Context) -> Rc<Datatype> {
        Rc::new(UnsignedNumber::<T> {
            phantom: PhantomData
        })
    }
    fn rust_type(&self) -> Ident {
        // type_name is pretty safe. rust-lang/rfcs#1428
        Ident::new(unsafe { ::std::intrinsics::type_name::<T>() })
    }

    fn generate_read(&self, _: u64) -> Tokens {
        let size = std::mem::size_of::<T>();
        let ty = self.rust_type();
        quote! { byteorder::ReadBytesExt::read_uint::<byteorder::BigEndian>(read, #size)? as #ty }
    }
}

#[derive(Debug)]
struct PString {
    count: Count
}
/*#[derive(Debug)]
struct CString;*/

impl Datatype for PString {
    fn create_from_fields(name: &str, val: &Value, generators: &mut Context) -> Rc<Datatype> {
        let opts = val.as_object().unwrap();
        let count = if opts.contains_key("count") {
            Count::from_val(&opts["count"])
        } else if opts.contains_key("countType") {
            Count::Type(generators.create_datatype(&(name.to_string() + "__count"), &opts["countType"]))
        } else {
            panic!();
        };
        Rc::new(PString {
            count: count
        })
    }

    fn rust_type(&self) -> Ident {
        Ident::new("String")
    }

    fn generate_read(&self, depth: u64) -> Tokens {
        let size_read = self.count.generate_read(depth);
        quote! { {
            let mut v = String::with_capacity(#size_read as usize);
            read.read_to_string(&mut v);
            v
        } }
    }

    fn generate_types(&self) -> Tokens {
        self.count.generate_types()
    }
}

/*impl Datatype for CString {
    fn create_from_fields(_: &str, _: &Value, _: &mut Context) -> Rc<Datatype> {
        Rc::new(CString)
    }

    fn rust_type(&self) -> Ident {
        Ident::new("String")
    }

    fn generate_read(&self, _: u64) -> Tokens {
        quote! { {
            let mut v = 
            read.read_until(0, &mut v)?;
            v
        } }
    }
}*/

#[derive(Debug)]
struct Field {
    name: String,
    ty: Rc<Datatype>
}

#[derive(Debug)]
struct Container {
    fields: Vec<Field>,
    type_name: String
}

impl Datatype for Container {
    fn create_from_fields(type_name: &str, fields: &Value, generators: &mut Context) -> Rc<Datatype> {
        let cont_fields = fields.as_array().unwrap().iter().map(|field| {
            let name = field["name"].as_str().unwrap();
            Field {
                name: name.to_string(),
                ty: generators.create_datatype(&(type_name.to_string() + "__" + name), &field["type"]),
            }
        }).collect();
        Rc::new(Container {
            fields: cont_fields,
            type_name: type_name.to_string()
        })
    }

    fn rust_type(&self) -> Ident {
        Ident::new(self.type_name.as_ref())
    }

    fn generate_read(&self, mut depth: u64) -> Tokens {
        depth += 1;
        let read_vars = self.fields.iter().map(|val| {
            // TODO: Support anon.
            let name = Ident::new(val.name.clone() + "_" + &depth.to_string());
            let generated_read = val.ty.generate_read(depth);
            quote! { let #name = #generated_read; }
        });
        let read_types = self.fields.iter().map(|val| {
            let name = Ident::new(val.name.as_ref());
            let var_name = Ident::new(val.name.clone() + "_" + &depth.to_string());
            quote! { #name: #var_name }
        });
        let name = self.rust_type();
        quote! { {
            #(#read_vars)*
            #name {
                #(#read_types),*
            }
        } }
    }

    fn generate_types(&self) -> Tokens {
        let name = self.rust_type();
        let other_structs : Vec<Tokens> = self.fields.iter().enumerate().map(|(idx, val)| {
            val.ty.generate_types()
        }).collect();
        let types : Vec<Tokens> = self.fields.iter().enumerate().map(|(idx, val)| {
            // TODO: Support anon.
            let name = Ident::new(val.name.as_ref());
            let ty = val.ty.rust_type();
            quote! { #name: #ty }
        }).collect();
        quote! {
            #(#other_structs)*
            #[derive(Debug)]
            struct #name {
                #(#types),*
            }
        }
    }
}

#[derive(Debug)]
struct Mapper {
    ty: Rc<Datatype>,
    // Technically, the key is a type that impls FromString...
    mappings: LinkedHashMap<String, String>
}

impl Datatype for Mapper {
    fn create_from_fields(name: &str, fields: &Value, generators: &mut Context) -> Rc<Datatype> {
        Rc::new(Mapper {
            ty: generators.create_datatype(name, &fields["type"]),
            mappings: fields["mappings"].as_object().unwrap().iter().map(|(k,v)| {
                (k.clone(), v.as_str().unwrap().to_string())
            }).collect()
        })
    }

    fn rust_type(&self) -> Ident {
        Ident::new("&'static str")
    }

    fn generate_read(&self, depth: u64) -> Tokens {
        let generated_read = self.ty.generate_read(depth);
        let vals = self.mappings.iter().map(|(k,v)| {
            // TODO: This only supports numbers. We need a better system.
            let ident_k = Ident::new(k.as_ref());
            quote! { #ident_k => #v }
        }).chain(std::iter::once(quote! { _ => panic!("WTF") }));
        quote! {{
            match #generated_read {
                #(#vals),*
            }
        }}
    }
}

#[derive(Debug)]
struct Switch {
    compare_to: String,
    fields: LinkedHashMap<String, Rc<Datatype>>,
    type_name: String
}

impl Datatype for Switch {
    fn create_from_fields(name: &str, args: &Value, generators: &mut Context) -> Rc<Datatype> {
        Rc::new(Switch {
            compare_to: args["compareTo"].as_str().unwrap().into(),
            fields: args["fields"].as_object().unwrap().iter().map(|(k, v)| {
                (k.clone(), generators.create_datatype(&(name.to_string() + "__" + k), &v))
            }).collect(),
            type_name: name.to_string()
        })
    }

    fn rust_type(&self) -> Ident {
        Ident::new(self.type_name.clone())
    }

    fn generate_read(&self, depth: u64) -> Tokens {
        let compare_to = Ident::new(fieldref(&self.compare_to, depth));
        let ty = self.rust_type();
        let fields = self.fields.iter().map(|(k, v)| {
            let k_ident = if let Some(true) = k.chars().nth(0).map(|c| !c.is_alphabetic()) {
                Ident::new(format!("_{}", k))
            } else {
                Ident::new(k.clone())
            };
            let generated_read = v.generate_read(depth);
            quote! { #k => #ty::#k_ident(#generated_read) }
        }).chain(std::iter::once(quote! { _ => panic!("WTF") }));
        quote! { match #compare_to {
            #(#fields),*
        }}
    }

    fn generate_types(&self) -> Tokens {
        let other_types = self.fields.iter().map(|(k, v)| {
            v.generate_types()
        });
        let fields = self.fields.iter().map(|(k, v)| {
            let k_ident = if let Some(true) = k.chars().nth(0).map(|c| !c.is_alphabetic()) {
                Ident::new(format!("_{}", k))
            } else {
                Ident::new(k.clone())
            };
            let ty = v.rust_type();
            quote! { #k_ident(#ty) }
        });
        let ty = Ident::new(self.type_name.clone());
        quote! {
            #(#other_types)*
            #[derive(Debug)]
            enum #ty {
                #(#fields),*
            }
        }
    }
}

#[derive(Debug)]
struct Array {
    inner_type: Rc<Datatype>,
    count: Count
}

impl Datatype for Array {
    fn create_from_fields(name: &str, val: &Value, generators: &mut Context) -> Rc<Datatype> {
        let opts = val.as_object().unwrap();
        let count = if opts.contains_key("count") {
            Count::from_val(&opts["count"])
        } else if opts.contains_key("countType") {
            Count::Type(generators.create_datatype(&(name.to_string() + "__count"), &opts["countType"]))
        } else {
            panic!();
        };
        Rc::new(Array {
            count: count,
            inner_type: generators.create_datatype(&(name.to_string() + "__inner"), &opts["type"])
        })
    }

    fn rust_type(&self) -> Ident {
        Ident::new(format!("Vec<{}>", self.inner_type.rust_type()))
    }

    fn generate_read(&self, depth: u64) -> Tokens {
        let count = self.count.generate_read(depth);
        let inner_read = self.inner_type.generate_read(depth);
        quote! { {
            let count = #count as usize;
            let mut v = Vec::with_capacity(count);
            for i in 0..count {
                v.insert(i, #inner_read);
            }
            v
        } }
    }

    fn generate_types(&self) -> Tokens {
        let count_type = self.count.generate_types();
        let inner_type = self.inner_type.generate_types();
        quote! {
            #count_type
            #inner_type
        }
    }
}

#[derive(Debug)]
struct Todo;

impl Datatype for Todo {
    fn create_from_fields(_: &str, _: &Value, _: &mut Context) -> Rc<Datatype> {
        Rc::new(Todo)
    }

    fn rust_type(&self) -> Ident {
        Ident::new("()")
    }

    fn generate_read(&self, _: u64) -> Tokens {
        quote!{ () }
    }
}

/// Gets the type name from a JSON Type Value.
// TODO: Return Result<>, exterminate all panics
fn get_type_name<'a>(val: &'a Value) -> &'a str {
    match *val {
        Value::String(ref s) => s,
        Value::Array(ref arr) => arr[0].as_str().unwrap(),
        Value::Object(_) => val["type"].as_str().unwrap(),
        _ => panic!()
    }
}

/// Gets the typeArgs from a JSON Type Value.
fn get_type_args(val: &Value) -> Value {
    match *val {
        Value::String(ref s) => Value::Null,
        Value::Array(ref arr) => arr[1].clone(),
        Value::Object(_) => val["typeArgs"].clone(),
        _ => panic!()
    }
}

fn generate_protodef(protodef_path: &str, mut types: Context, packet_str: &str) -> Tokens {
    types.json = serde_json::from_reader(File::open(protodef_path).unwrap()).unwrap();
    //let crates = vec![];
    let packet_type = types.create_datatype(packet_str, &Value::String(packet_str.into()));
    let ty = packet_type.rust_type();
    let newtypes = packet_type.generate_types();
    let generated_read = packet_type.generate_read(0);
    let mut toks = quote!{
        extern crate byteorder;

        #newtypes
        fn read(read: &mut ::std::io::Read) -> std::io::Result<#ty> {
            let res = #generated_read;
            Ok(res)
        }
    };
    return toks;
}

fn main() {
    let mut types = Context {
        type_constructors: HashMap::new(),
        json: Value::Null,
        types: HashMap::new()
    };
    types.register::<SignedNumber<i8>>("i8");
    types.register::<SignedNumber<i16>>("i16");
    types.register::<SignedNumber<i32>>("i32");
    types.register::<SignedNumber<i64>>("i64");
    types.register::<UnsignedNumber<u8>>("u8");
    types.register::<UnsignedNumber<u16>>("u16");
    types.register::<UnsignedNumber<u32>>("u32");
    types.register::<UnsignedNumber<u64>>("u64");
    types.register::<Container>("container");
    types.register::<Mapper>("mapper");
    types.register::<Switch>("switch");
    types.register::<Array>("array");
    /*types.register::<Magic>("magic");*/
    types.register::<PString>("pstring");
    /*types.register::<RestBuffer>("restBuffer");
    types.register::<IPAddress>("ipAddress");
    types.register::<LTriad>("ltriad");
    types.register::<EndOfArray>("endOfArray");*/
    let _in = generate_protodef("protocol.json", types, "packet").to_string();


    let mut out : Vec<u8> = vec![];
    let mut config = rustfmt::config::Config::default();
    config.write_mode = rustfmt::config::WriteMode::Plain;
    rustfmt::format_input(rustfmt::Input::Text(_in), &config, Some(&mut out)).unwrap();
    println!("{}", String::from_utf8(out).unwrap());
}

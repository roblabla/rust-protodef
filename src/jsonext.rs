use serde_json::{Value, Map};

fn parse_index(s: &str) -> Option<usize> {
    if s.starts_with('+') || (s.starts_with('0') && s.len() != 1) {
        return None;
    }
    s.parse().ok()
}

pub trait ValueExt {
    fn pointer_mut_create<'a>(&'a mut self, pointer: &str) -> Option<&'a mut Value>;
}

impl ValueExt for Value {
    fn pointer_mut_create<'a>(&'a mut self, pointer: &str) -> Option<&'a mut Value> {
        if pointer == "" {
            return Some(self);
        }
        if !pointer.starts_with('/') {
            panic!();
        }
        let tokens = pointer.split('/').skip(1).map(|x| x.replace("~1", "/").replace("~0", "~"));
        let mut target = self;

        for token in tokens {
            // borrow checker gets confused about `target` being mutably borrowed too many times because of the loop
            // this once-per-loop binding makes the scope clearer and circumvents the error
            let target_once = target;
            let target_opt = match *target_once {
                Value::Object(ref mut map) =>
                    Some(map.entry(token).or_insert(Value::Object(Map::new()))),
                Value::Array(ref mut list) =>
                    parse_index(&token).and_then(move |x| list.get_mut(x)),
                _ => return None,
            };
            if let Some(t) = target_opt {
                target = t;
            } else {
                return None;
            }
        }
        Some(target)
    }
}

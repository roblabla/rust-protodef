extern crate byteorder;

use std::io::{Read, Write, Result};
use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian};

/*trait Datatype : Sized {
    fn read_from(&mut Read) -> Result<Self>;
    fn write_to(self, &mut Write) -> Result<()>;
}

macro_rules! impl_read {
    ( $(($x:ty, $read:tt, $write:tt)),* ) => {
        $(impl Datatype for $x {
            fn read_from(read: &mut Read) -> Result<Self> {
                read.$read()
            }

            fn write_to(self, write: &mut Write) -> Result<()> {
                write.$write(self)
            }
        })*
    }
}

impl_read! { (i8, read_i8, write_i8), (u8, read_u8, write_u8),
             (i16, read_i16::<BigEndian>, write_i16::<BigEndian>),
             (u16, read_u16::<BigEndian>, write_u16::<BigEndian>),
             (i32, read_i32::<BigEndian>, write_i32::<BigEndian>),
             (u32, read_u32::<BigEndian>, write_u32::<BigEndian>),
             (i64, read_i64::<BigEndian>, write_i64::<BigEndian>),
             (u64, read_u64::<BigEndian>, write_u64::<BigEndian>)
}

impl Datatype for u8 {
    fn read_from(read: &mut Read) -> Result<Self> {
        read.read_u8()
    }

    fn write_to(self, write: &mut Write) -> Result<()> {
        write.write_u8(self)
    }
}

impl Datatype for i8 {
    fn read_from(read: &mut Read) -> Result<Self> {
        read.read_i8()
    }

    fn write_to(self, write: &mut Write) -> Result<()> {
        write.write_i8(self)
    }
}
*/
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

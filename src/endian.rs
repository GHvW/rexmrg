use crate::read_bytes::FromBytes;
use std::io;
use std::io::prelude::*;

#[derive(Debug, Copy, Clone)]
pub enum Endian {
    Little,
    Big,
}

// figure out if there is a way to consolidate some of these with generics. A type that all from_xx_bytes implement
impl Endian {
    pub fn read<T: FromBytes>(self, reader: &mut impl Read) -> io::Result<T> {
        T::from_bytes(self, reader)
    }
}

fn read_b_int32<R: Read>(reader: &mut R) -> io::Result<i32> {
    let mut buffer = [0; 4];
    reader.read_exact(&mut buffer)?; // need error handling in case not 4 bytes?

    Ok(i32::from_be_bytes(buffer))
}

pub fn get_endian<R: Read>(reader: &mut R) -> io::Result<Endian> {
    let word = read_b_int32(reader);

    word.and_then(|int| match int {
        16 => Ok(Endian::Big),
        _ => Ok(Endian::Little),
    })
}

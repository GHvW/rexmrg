use crate::endian::Endian;

use std::io;
use std::io::prelude::*;

pub trait FromBytes: Sized {
    fn from_bytes(endianness: Endian, reader: &mut impl Read) -> io::Result<Self>;
}

impl FromBytes for u8 {
    fn from_bytes(endianness: Endian, reader: &mut impl Read) -> io::Result<Self> {
        let mut buffer = [0; 1];
        reader.read_exact(&mut buffer)?;
        match endianness {
            Endian::Big => Ok(u8::from_be_bytes(buffer)),
            Endian::Little => Ok(u8::from_le_bytes(buffer)),
        }
    }
}

impl FromBytes for i16 {
    fn from_bytes(endianness: Endian, reader: &mut impl Read) -> io::Result<Self> {
        let mut buffer = [0; 2];
        reader.read_exact(&mut buffer)?;
        match endianness {
            Endian::Big => Ok(i16::from_be_bytes(buffer)),
            Endian::Little => Ok(i16::from_le_bytes(buffer)),
        }
    }
}

impl FromBytes for i32 {
    fn from_bytes(endianness: Endian, reader: &mut impl Read) -> io::Result<Self> {
        let mut buffer = [0; 4];
        reader.read_exact(&mut buffer)?;
        match endianness {
            Endian::Big => Ok(i32::from_be_bytes(buffer)),
            Endian::Little => Ok(i32::from_le_bytes(buffer)),
        }
    }
}

impl FromBytes for f32 {
    fn from_bytes(endianness: Endian, reader: &mut impl Read) -> io::Result<Self> {
        let mut buffer = [0; 4];
        reader.read_exact(&mut buffer)?;
        match endianness {
            Endian::Big => Ok(f32::from_be_bytes(buffer)),
            Endian::Little => Ok(f32::from_le_bytes(buffer)),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ReadBytes {
    count: i32,
    endian: Endian,
}

// figure out if there is a way to consolidate some of these into one using generics
// does self need to be a reference or can we consume it?
impl ReadBytes {
    pub fn new(count: i32, endian: Endian) -> Self {
        Self { count, endian }
    }

    pub fn iter<'a, T: FromBytes, R: Read>(
        self,
        reader: &'a mut R,
    ) -> impl Iterator<Item = io::Result<T>> + 'a {
        (0..self.count).map(move |_| T::from_bytes(self.endian, reader))
    }

    // The following are convenience methods so you don't need to write collect::<io::Result<Vec<TYPE>>>() when you just want the bytes in a Vec

    pub fn read_int32s<R: Read>(self, reader: &mut R) -> io::Result<Vec<i32>> {
        self.iter(reader).collect()
    }

    pub fn read_int16s<R: Read>(self, reader: &mut R) -> io::Result<Vec<i16>> {
        self.iter(reader).collect()
    }

    pub fn read_u8s<R: Read>(self, reader: &mut R) -> io::Result<Vec<u8>> {
        self.iter(reader).collect()
    }
}

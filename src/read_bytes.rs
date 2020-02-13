use crate::endian::Endian;

use std::io;
use std::io::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct ReadBytes {
    count: i32,
    endian: Endian
}

// figure out if there is a way to consolidate some of these into one using generics
// does self need to be a reference or can we consume it?
impl ReadBytes {

    pub fn new(count: i32, endian: Endian) -> Self {
        Self { count, endian }
    }

    pub fn iter_int32s<'a, R: Read>(self, reader: &'a mut R) -> impl Iterator<Item=io::Result<i32>> + 'a {
        (0..self.count).map(move |_| {
            self.endian.read_int32(reader)
        })
    }

    pub fn iter_int16s<'a, R: Read>(self, reader: &'a mut R) -> impl Iterator<Item=io::Result<i16>> + 'a {
        (0..self.count).map(move |_| {
            self.endian.read_int16(reader)
        })
    }

    pub fn iter_u8s<'a, R: Read>(self, reader: &'a mut R) -> impl Iterator<Item=io::Result<u8>> + 'a {
        (0..self.count).map(move |_| {
            self.endian.read_u8(reader)
        })
    }

    // The following are convenience methods so you don't need to write collect::<io::Result<Vec<TYPE>>>() when you just want the bytes in a Vec

    pub fn read_int32s<R: Read>(self, reader: &mut R) -> io::Result<Vec<i32>> {
        self.iter_int32s(reader).collect()
    }

    pub fn read_int16s<R: Read>(self, reader: &mut R) -> io::Result<Vec<i16>> {
        self.iter_int16s(reader).collect()
    }

    pub fn read_u8s<R: Read>(self, reader: &mut R) -> io::Result<Vec<u8>> {
        self.iter_u8s(reader).collect()
    }
}
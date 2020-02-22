use std::io;
use std::io::prelude::*;

#[derive(Debug, Copy, Clone)]
pub enum Endian {
    Little,
    Big,
}

// figure out if there is a way to consolidate some of these with generics. A type that all from_xx_bytes implement
impl Endian {
    pub fn read_int32<R: Read>(&self, reader: &mut R) -> io::Result<i32> {
        let mut buffer = [0; 4];
        reader.read_exact(&mut buffer)?; // need error handling in case not 4 bytes?

        match self {
            Endian::Big => Ok(i32::from_be_bytes(buffer)),
            Endian::Little => Ok(i32::from_le_bytes(buffer)),
        }
    }

    pub fn read_int16<R: Read>(&self, reader: &mut R) -> io::Result<i16> {
        let mut buffer = [0; 2];
        reader.read_exact(&mut buffer)?; // need error handling in case not 4 bytes?

        match self {
            Endian::Big => Ok(i16::from_be_bytes(buffer)),
            Endian::Little => Ok(i16::from_le_bytes(buffer)),
        }
    }

    pub fn read_u8<R: Read>(&self, reader: &mut R) -> io::Result<u8> {
        let mut buffer = [0; 1];
        reader.read_exact(&mut buffer)?; // need error handling in case not 4 bytes?

        match self {
            Endian::Big => Ok(u8::from_be_bytes(buffer)),
            Endian::Little => Ok(u8::from_le_bytes(buffer)),
        }
    }

    pub fn read_f32<R: Read>(&self, reader: &mut R) -> io::Result<f32> {
        let mut buffer = [0; 4];
        reader.read_exact(&mut buffer)?; // need error handling in case not 4 bytes?

        match self {
            Endian::Big => Ok(f32::from_be_bytes(buffer)),
            Endian::Little => Ok(f32::from_le_bytes(buffer)),
        }
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

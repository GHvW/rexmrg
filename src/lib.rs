use std::io;
use std::io::prelude::*;
use std::io::{BufReader};
use std::fs::File;
// use std::fs;
// use std::convert::TryInto;
// use std::io::SeekFrom;
// use std::ops::Range;

//https://www.nws.noaa.gov/oh/hrl/dmip/2/xmrgformat.html
// https://www.nws.noaa.gov/oh/hrl/misc/xmrg.pdf
// https://www.nws.noaa.gov/oh/hrl/dmip/2/src/read_xmrg2.c

pub enum Endian {
    Little,
    Big
}

impl Endian {
    pub fn read_int32<R: Read>(&self, reader: &mut R) -> io::Result<i32> {
        let mut buffer = [0; 4];
        reader.read_exact(&mut buffer)?; // need error handling in case not 4 bytes?

        match self {
            Endian::Big => Ok(i32::from_be_bytes(buffer)),
            Endian::Little => Ok(i32::from_le_bytes(buffer))
        }
    }
}

pub fn read_b_int32<R: Read>(reader: &mut R) -> io::Result<i32> {

    let mut buffer = [0; 4];
    reader.read_exact(&mut buffer)?; // need error handling in case not 4 bytes?

    Ok(i32::from_be_bytes(buffer))
}

pub fn get_endian<R: Read>(reader: &mut R) -> io::Result<Endian> {
    let word = read_b_int32(reader);

    word.and_then(|int| {
        match int {
            16 => Ok(Endian::Big),
            _ => Ok(Endian::Little)
        }
    })
}

pub struct ReadBytes {
    count: u64,
    endian: Endian
}

impl ReadBytes {

    pub fn new(count: u64, endian: Endian) -> Self {
        Self { count, endian }
    }

    pub fn read_int32s<R: Read>(&self, reader: &mut R) -> io::Result<Vec<i32>> {
        (0..self.count).map(|_| {
            // reader.seek(SeekFrom::Current(i))?;
            self.endian.read_int32(reader)
        })
        .collect()
    }
}

// pub fn read_int32s(reader: &mut BufReader<File>) -> io::Result<Vec<i32>> {
//     // let _ = reader.seek(SeekFrom::Start(4))?;

//     (0..4).map(|_| {
//         // reader.seek(SeekFrom::Current(i))?;
//         read_int32(reader)
//     })
//     .collect()
// }

pub fn get_reader(path: &str) -> io::Result<BufReader<File>> {
    let file = File::open(path)?;
    Ok(BufReader::new(file))
}

// pub fn read_xmrg(file_path: &str) -> io::Result<()> {
//     let file = File::open(file_path)?;
//     let mut reader = BufReader::new(file);
//     let mut num_bytes = [0; 1];
    
//     let mut handle = reader.take(1);
    
//     handle.read(&mut num_bytes)?;
//     let _needs_reversal = need_byte_reversal(num_bytes[0].try_into().unwrap());
    
//     Ok(())
// }

trait ByteReader {
    fn as_int32(&self) -> i32;
}

impl ByteReader for [u8; 4] {
    fn as_int32(&self) -> i32 {
        i32::from_le_bytes(*self)
    }
}



// impl ByteReader for Vec<u8> {
//     fn read_int32(&self) -> i32 {
//         i32::from_le_bytes(*self)
//     }
// }
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_int32_test() {
        let buff = [1, 1, 1, 1];
        let i = buff.as_int32();
        assert_eq!(i, 16_843_009);
    }
}
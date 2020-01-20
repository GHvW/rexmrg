use std::io;
use std::io::prelude::*;
use std::io::{BufReader};
use std::fs::File;
// use std::fs;
// use std::convert::TryInto;
use std::io::SeekFrom;
// use std::ops::Range;

//https://www.nws.noaa.gov/oh/hrl/dmip/2/xmrgformat.html
// https://www.nws.noaa.gov/oh/hrl/misc/xmrg.pdf
// https://www.nws.noaa.gov/oh/hrl/dmip/2/src/read_xmrg2.c
// https://www.nws.noaa.gov/oh/hrl/distmodel/hrap.htm
// https://www.nws.noaa.gov/oh/hrl/gis/hrap/xmrgtolist.c
// https://www.nws.noaa.gov/oh/hrl/gis/hrap/xmrgtoasc.c
// HRAP https://www.nws.noaa.gov/oh/hrl/distmodel/hrap.htm

const XOR: usize = 0;
const YOR: usize = 1;
const COLUMNS: usize = 2;
const ROWS: usize = 3;

pub fn get_reader(path: &str) -> io::Result<BufReader<File>> {
    let file = File::open(path)?;
    Ok(BufReader::new(file))
}


pub fn read_b_int32<R: Read>(reader: &mut R) -> io::Result<i32> {

    let mut buffer = [0; 4];
    reader.read_exact(&mut buffer)?; // need error handling in case not 4 bytes?

    Ok(i32::from_be_bytes(buffer))
}


#[derive(Debug, Copy, Clone)]
pub enum Endian {
    Little,
    Big
}

// figure out if there is a way to consolidate some of these with generics. A type that all from_xx_bytes implement
impl Endian {
    pub fn read_int32<R: Read>(&self, reader: &mut R) -> io::Result<i32> {
        let mut buffer = [0; 4];
        reader.read_exact(&mut buffer)?; // need error handling in case not 4 bytes?

        match self {
            Endian::Big => Ok(i32::from_be_bytes(buffer)),
            Endian::Little => Ok(i32::from_le_bytes(buffer))
        }
    }

    pub fn read_int16<R: Read>(&self, reader: &mut R) -> io::Result<i16> {
        let mut buffer = [0; 2];
        reader.read_exact(&mut buffer)?; // need error handling in case not 4 bytes?

        match self {
            Endian::Big => Ok(i16::from_be_bytes(buffer)),
            Endian::Little => Ok(i16::from_le_bytes(buffer))
        }
    }

    pub fn read_u8<R: Read>(&self, reader: &mut R) -> io::Result<u8> {
        let mut buffer = [0; 1];
        reader.read_exact(&mut buffer)?; // need error handling in case not 4 bytes?

        match self {
            Endian::Big => Ok(u8::from_be_bytes(buffer)),
            Endian::Little => Ok(u8::from_le_bytes(buffer))
        }
    }
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

// see the second record section of https://www.nws.noaa.gov/oh/hrl/misc/xmrg.pdf
#[derive(Debug)]
pub enum XmrgVersion {
    Pre1997,
    Build4_2,
    Build5_2_2,
}


pub fn get_xmrg_version(byte_count: i32, max_x: i32) -> Option<XmrgVersion> {
    match byte_count {
        66 => Some(XmrgVersion::Build5_2_2),
        38 => Some(XmrgVersion::Build4_2), // a 37 byte version may be valid. Consider adding
        n if n == max_x * 2 => Some(XmrgVersion::Pre1997),
        _ => None
    }
}


// if a data point is negative, represent as -999 (no data), if positive, divide by 100 to represent in millimeters
// data points are represented as a 100th of a milimeter. .001mm is represented as 1 in a xmrg data point, dividing by 100 gets us to .001
pub fn to_mm(data_point: i16) -> f64 {
    if data_point < 0 {
        -999.0
    } else {
        data_point as f64 / 100.0
    }
}

pub fn process_row<R: Read + Seek>(read_bytes: ReadBytes, reader: &mut R) -> io::Result<Vec<f64>> {
    reader.seek(SeekFrom::Current(4))?;

    let result = 
        read_bytes
            .iter_int16s(reader)
            .map(|res| res.map(to_mm))
            .collect();

    reader.seek(SeekFrom::Current(4))?;

    result
}

pub fn read_xmrg(path: &str) -> io::Result<Vec<Vec<f64>>> {
    let mut reader = get_reader(path)?;
    let endian = get_endian(&mut reader)?;

    let header = ReadBytes::new(4,endian).read_int32s(&mut reader)?;
    reader.seek(SeekFrom::Current(4))?;

    let record_2_bytes = endian.read_int32(&mut reader)?;

    let xmrg_version = get_xmrg_version(record_2_bytes, header[COLUMNS]);

    let row_reader = ReadBytes::new(header[COLUMNS], endian);

    let result = xmrg_version.map(|version| {
        match version {
            XmrgVersion::Pre1997 => {
                reader.seek(SeekFrom::Start(24))?; // set reader to position just after header (4 bytes + 16 byte header + 4 bytes = 24) 

                (0..header[ROWS]).map(|_| {
                    process_row(row_reader, &mut reader)
                })
                .collect()
                // let first_row = 
                //     row_reader
                //         .iter_int16s(&mut reader)
                //         .map(|res| res.map(to_mm))
                //         .collect::<io::Result<Vec<f64>>>()?;
                
                // reader.seek(SeekFrom::Current(4))?;

                // let mut rows = vec![first_row];
                // // (0..1).map(|_| {
                // //     first_row
                // // })
                // // .chain(
                // //     (0..header[ROWS - 1]).map(|_i| {
                // //         process_row(row_reader, &mut reader)
                // //     })
                // // )
                // // .collect::<io::Result<Vec<Vec<f64>>>>()
                // for _ in 0..header[ROWS] - 1 {
                //     let row = process_row(row_reader, &mut reader)?;
                //     rows.push(row);
                // }

                // Ok(rows)
            },
            _ => Ok(Vec::new()) // not implemented
        }
    })
    .expect("Could not determine XMRG version");

    result
}

pub struct Point {
    x: i32,
    y: i32
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

pub struct Header {
    xor: i32,
    yor: i32,
    columns: i32,
    rows: i32
}

impl Header {
    pub fn from_vec(vec: Vec<i32>) -> Self {
        Self {
            xor: vec[XOR],
            yor: vec[YOR],
            columns: vec[COLUMNS],
            rows: vec[ROWS]
        }
    }

    
    pub fn generate_coordinates(&self) -> Vec<Vec<Point>> {
        (self.xor..self.rows).map(|y| {
            (self.yor..self.columns).map(|x| {
                Point::new(x, y)
            })
            .collect()
        })
        .collect()
    }
}


#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn as_int32_test() {
    //     let buff = [1, 1, 1, 1];
    //     let i = buff.as_int32();
    //     assert_eq!(i, 16_843_009);
    // }
    #[test]
    fn le_be_ne_test() {
        let buff = [0b10101010, 0b11100101];

        assert_eq!(u16::from_be_bytes(buff), 0b1010101011100101);
        assert_eq!(u16::from_le_bytes(buff), 0b1110010110101010);
        assert_eq!(u16::from_ne_bytes(buff), 0b1110010110101010);
    }
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

// impl ByteReader for Vec<u8> {
//     fn read_int32(&self) -> i32 {
//         i32::from_le_bytes(*self)
//     }
// }
pub mod endian;
pub mod geo;
pub mod headers;
pub mod hrap;
pub mod read_bytes;
pub mod utils;
pub mod xmrg_version;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::SeekFrom;

use endian::get_endian;
use geo::Feature;
use headers::{Header, Metadata};
use read_bytes::ReadBytes;
use utils::to_mm;
use xmrg_version::{get_xmrg_version, XmrgVersion};

// https://tgftp.nws.noaa.gov/data/rfc/wgrfc/
// https://www.nws.noaa.gov/oh/hrl/dmip/2/xmrgformat.html
// https://www.nws.noaa.gov/oh/hrl/misc/xmrg.pdf
// https://www.nws.noaa.gov/oh/hrl/dmip/2/src/read_xmrg2.c
// https://www.nws.noaa.gov/oh/hrl/distmodel/hrap.htm
// https://www.nws.noaa.gov/oh/hrl/gis/hrap/xmrgtolist.c
// https://www.nws.noaa.gov/oh/hrl/gis/hrap/xmrgtoasc.c
// HRAP https://www.nws.noaa.gov/oh/hrl/distmodel/hrap.htm
// HRAP function https://www.nws.noaa.gov/oh/hrl/dmip/lat_lon.txt

// const XOR: usize = 0;
// const YOR: usize = 1;
const COLUMNS: usize = 2;
const ROWS: usize = 3;

fn get_reader(path: &str) -> io::Result<BufReader<File>> {
    let file = File::open(path)?;
    Ok(BufReader::new(file))
}

fn process_row<R: Read + Seek>(read_bytes: ReadBytes, reader: &mut R) -> io::Result<Vec<f64>> {
    reader.seek(SeekFrom::Current(4))?;

    let result = read_bytes
        .iter_int16s(reader)
        .map(|res| res.map(to_mm))
        .collect();

    reader.seek(SeekFrom::Current(4))?;

    result
}

// pub fn read_xmrg(path: &str) -> io::Result<Vec<Vec<f64>>> {
pub fn read_xmrg(path: &str) -> io::Result<XmrgData> {
    let mut reader = get_reader(path)?;
    let endian = get_endian(&mut reader)?;

    let header = ReadBytes::new(4, endian).read_int32s(&mut reader)?;
    reader.seek(SeekFrom::Current(4))?;

    let record_2_bytes = endian.read_int32(&mut reader)?;

    let xmrg_version = get_xmrg_version(record_2_bytes, header[COLUMNS]);

    let row_reader = ReadBytes::new(header[COLUMNS], endian);

    let values = xmrg_version.map_or(Ok(Vec::new()), |version| {
        match version {
            XmrgVersion::Pre1997 => {
                reader.seek(SeekFrom::Start(24))?; // set reader to position just after header (4 bytes + 16 byte header + 4 bytes = 24)

                (0..header[ROWS])
                    .map(|_| process_row(row_reader, &mut reader))
                    .collect()
            }
            // XmrgVersion::Build4_2 => {

            // },
            // XmrgVersion::Build5_2_2 => {

            // },
            _ => Ok(Vec::new()), // not implemented
        }
    })?;

    Ok(XmrgData::new(Header::from_vec(header), None, values))
}

pub struct XmrgData {
    pub header: Header,
    pub metadata: Option<Metadata>,
    pub values: Vec<Vec<f64>>,
}

impl XmrgData {
    pub fn new(header: Header, metadata: Option<Metadata>, values: Vec<Vec<f64>>) -> Self {
        XmrgData {
            header,
            metadata,
            values,
        }
    }

    // https://github.com/rust-lang/rfcs/blob/master/text/1951-expand-impl-trait.md#scoping-for-type-and-lifetime-parameters
    // pub fn generate_features<'a>(&'a self) -> impl Iterator<Item=Feature> + 'a {
    pub fn generate_features(&self) -> impl Iterator<Item = Feature> + '_ {
        self.values
            .iter()
            .flat_map(|vec| vec.iter())
            .zip(self.header.into_iter())
            .map(|(value, point)| Feature::new(point, *value))
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

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

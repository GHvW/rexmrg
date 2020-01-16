use std::io;
use std::io::prelude::*;
use std::io::{BufReader};
use std::fs::File;
// use std::convert::TryInto;
// use std::io::SeekFrom;
// use std::ops::Range;

//https://www.nws.noaa.gov/oh/hrl/dmip/2/xmrgformat.html
// https://www.nws.noaa.gov/oh/hrl/misc/xmrg.pdf
// https://www.nws.noaa.gov/oh/hrl/dmip/2/src/read_xmrg2.c


pub fn need_byte_reversal(num_bytes: u32) -> bool {
    if num_bytes != 16 { true } else { false }
}

// fn reverse_byte_order(arr: Vec<i32>) -> Vec<i32> {
//     arr.iter()
//         .
// }
// pub fn header_contents(reader: &mut BufReader<File>) -> io::Result<[u32; 4]> {
//     reader.seek(SeekFrom::Start(4))?;
//     let mut handle = reader.take(4);

//     // header size 4
//     let header = (0..4).fold([0; 4], |arr, i| {
//         arr[i] = read_int32(reader).unwrap();
//         arr
//     }).collect();
    
//     Ok(header)
// }

// pub fn read_int32(reader: &mut BufReader<File>) -> io::Result<(i32, BufReader<File>)> {
pub fn read_int32<'a, R: Read + 'a>(reader: R) -> io::Result<(i32, Box<dyn Read + 'a>)> {

    let mut handle = reader.take(4);

    let mut buffer = [0; 4];
    handle.read(&mut buffer)?;

    // Intel uses little endian
    Ok((i32::from_le_bytes(buffer), Box::new(handle)))
}

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
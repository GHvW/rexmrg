use rexmrg::{ReadBytes, get_endian, get_reader, read_b_int32, get_xmrg_version};
use std::fs::File;
use std::io::{Read};
use std::io;
use std::io::SeekFrom;
use std::io::prelude::*;

fn main() {
    println!("Hello, world!");


    let mut reader = get_reader("xmrg0506199516z.gz").unwrap();
    let endian = get_endian(&mut reader).unwrap();
    let header_bytes = ReadBytes::new(4, endian);

    let header = header_bytes.read_int32s(&mut reader).unwrap();
    println!("the header is {:?}", &header);

    reader.seek(SeekFrom::Current(4)).unwrap();

    let num_row2_bytes = endian.read_int32(&mut reader).unwrap();
    // let num_header2_bytes = read_b_int32(&mut reader).unwrap();
    // let header2_bytes = ReadBytes::new(num_row2_bytes as u64, endian);
    let xmrg_version = get_xmrg_version(num_row2_bytes, header[2]);
    println!("xmrg version: {:?}", xmrg_version.unwrap());

    // println!("num row2 bytes {}", num_row2_bytes);

    // println!("the row 2 is {:?}", header2_bytes.read_u8s(&mut reader).unwrap());
    
    println!("now for the tester");

    // tester("xmrg0506199516z.gz", 32).unwrap();

    println!("Fin ...");
}

pub fn tester(path: &str, stop: usize) -> io::Result<()> {
    let file = File::open(path)?;

    for (i, b) in file.bytes().enumerate() {
        println!("byte {} is: {:b}", i, b.unwrap());
        if (i + 1) % 4 == 0 { println!("*********  INT!! *********"); }
        if i > 0 && (i - 1) == stop { break; }
    }

    Ok(())
}


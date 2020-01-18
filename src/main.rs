use rexmrg::{ReadBytes, get_endian, get_reader};
use std::fs::File;
use std::io::{Read};
use std::io;
use std::io::SeekFrom;
use std::io::prelude::*;

fn main() {
    println!("Hello, world!");


    let mut reader = get_reader("xmrg0506199516z.gz").unwrap();
    let endian = get_endian(&mut reader).unwrap();
    // reader.seek(SeekFrom::Current(4)).unwrap();
    let header_reader = ReadBytes::new(4, endian);

    println!("the header is {:?}", header_reader.read_int32s(&mut reader).unwrap());
    
    println!("now for the tester");

    tester("xmrg0506199516z.gz", 32).unwrap();

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


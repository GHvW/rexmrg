use rexmrg::{ReadBytes, get_endian, get_reader, get_xmrg_version};
use std::fs::File;
use std::io::{Read};
use std::io;
use std::io::SeekFrom;
use std::io::prelude::*;

fn main() {
    println!("Hello, world!");


    println!("------------ V1 --------------");
    // ********************* start v1 ******************************
    let mut reader = get_reader("xmrg0506199516z.gz").unwrap();
    let endian = get_endian(&mut reader).unwrap();
    let header_bytes = ReadBytes::new(4, endian);

    let header = header_bytes.read_int32s(&mut reader).unwrap();
    println!("the header is {:?}", &header);

    reader.seek(SeekFrom::Current(4)).unwrap();

    let num_row2_bytes = endian.read_int32(&mut reader).unwrap();

    let xmrg_version = get_xmrg_version(num_row2_bytes, header[2]);
    println!("xmrg version: {:?}", xmrg_version.unwrap());

    // ********************* end v1 ********************************

    println!("---------- V2 ---------------");
    // ********************* start v2 ******************************
    let mut reader2 = get_reader("xmrg0506199516z.gz").unwrap();
    let endian2 = get_endian(&mut reader2).unwrap();
    let header_bytes2 = ReadBytes::new(4, endian2);

    let header2 = header_bytes2.iter_int32s(&mut reader2).collect::<io::Result<Vec<i32>>>().unwrap();
    println!("the header is {:?}", &header2);

    reader2.seek(SeekFrom::Current(4)).unwrap();

    let num_row2_bytes2 = endian2.read_int32(&mut reader2).unwrap();

    let xmrg_version2 = get_xmrg_version(num_row2_bytes2, header2[2]);
    println!("xmrg version: {:?}", xmrg_version2.unwrap());

    let row1_2_bytes = ReadBytes::new(header2[2] as u64, endian2);
    let row1_in_mm = row1_2_bytes
        .iter_int16s(&mut reader2)
        .map(|res| {
            res.map(|item| {
                if item >= 0 { 
                    item as f64 / 100.0000000 // ** convert to mm from hundreth of mm **
                } else {
                    -999.000000
                }
            })
        })
        .collect::<io::Result<Vec<f64>>>()
        .unwrap();

    println!("row count is {}", row1_in_mm.len());
    println!("first row in mm?: {:?}", row1_in_mm);
    // ********************* end v2 ********************************

    
    println!("now for the tester");

    // tester("xmrg0506199516z.gz", 400).unwrap();

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


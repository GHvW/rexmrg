// use rexmrg::{ReadBytes, get_endian, get_reader, get_xmrg_version};
// use rexmrg::{read_xmrg, Header};
use rexmrg::{read_xmrg, hrap_to_latlon};
use std::fs::File;
use std::io::{Read};
use std::io;
use std::f64;
// use std::io::SeekFrom;
// use std::io::prelude::*;
// use std::io::{self, Write};

fn main() -> io::Result<()> {
    println!("Hello, world!");


    println!("------------ V1 --------------");
    // ********************* start v1 ******************************
    let xmrg_data = read_xmrg("xmrg0506199516z.gz").unwrap();

    // println!("{:?}", xmrg_data);

    let avg = average(&xmrg_data);
    let max = max(&xmrg_data);

    println!("The avg is {}", avg);
    println!("The max is {}", max);

    println!("long lat is {:?}", hrap_to_latlon(367.0, 263.0));
    // ********************* end v1 ********************************

    println!("---------- V2 ---------------");
    // ********************* start v2 ******************************

    // let test_header = Header::from_vec(vec![367, 263, 335, 159]);

    // println!("coordinates\n{:?}", test_header.generate_coordinates());

    // ********************* end v2 ********************************

    
    println!("now for the tester");

    // tester("xmrg0506199516z.gz", 400).unwrap();

    println!("Fin ...");

    // io::stdout().write_all(&row1_in_mm)?;

    Ok(())
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

//this is gross, do something about it later
pub fn average(data: &Vec<Vec<f64>>) -> f64 {
    // let count = (data.len() * data[0].len()) as f64;
    let mut count = 0;

    // data.iter()
    //     .flatten()
    //     .map(|x| if *x < 0.0 { 0.0 } else { *x })
    //     .sum::<f64>() / count
    data.iter()
        .flatten()
        .filter(|n| **n >= 0.0)
        .inspect(|_| count += 1)
        .sum::<f64>() / (count as f64)
}

pub fn max(data: &Vec<Vec<f64>>) -> f64 {
    data.iter()
        .flatten()
        .filter(|n| **n >= 0.0)
        .fold(f64::MIN, |current, n| current.max(*n))
}
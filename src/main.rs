use rexmrg::{read_int32, get_reader};



fn main() {
    println!("Hello, world!");
    let arr = [0b1, 0b1, 0b1, 0b1];
    // 1 1
    let value = u32::from_be_bytes(arr);

    println!("val {}", value);

    let reader = get_reader("xmrg0506199516z.gz");

    let rez1 = reader.and_then(read_int32);
    let rez2 = rez1.map(|(i, handle)| {
        println!("i is {}", i);
        handle
    })
    .and_then(read_int32)
    .map(|(i, _)| {
        println!("i 2 in is {}", i);
        i
    });

    println!("i 2 is {}", rez2.unwrap());
}

use rexmrg::{read_header, get_reader};

fn main() {
    println!("Hello, world!");

    // let rez2 = rez1.map(|(i, handle)| {
    //     println!("i is {}", i);
    //     handle
    // })
    // .and_then(read_int32)
    // .map(|(i, handle)| {
    //     println!("i 2 in is {}", i);
    //     handle
    // })
    // .and_then(read_int32)
    // .map(|(i, handle)| {
    //     println!("i 3 in is {}", i);
    //     handle
    // })
    // .and_then(read_int32)
    // .map(|(i, _)| {
    //     println!("i 3 in is {}", i);
    //     i
    // });

    // println!("i 3 is {}", rez2.unwrap());
    let header = read_header(&mut get_reader("xmrg0506199516z.gz").unwrap()).unwrap();
    println!("the header is {:?}", header);
    
    println!("now for the tester");

    // tester("xmrg0506199516z.gz", 20).unwrap();

    println!("Fin ...");
}

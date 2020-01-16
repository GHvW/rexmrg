fn main() {
    println!("Hello, world!");
    let arr = [0b1, 0b1, 0b1, 0b1];
    // 1 1
    let value = u32::from_be_bytes(arr);

    println!("val {}", value);
}

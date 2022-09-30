use std::io;

fn main() {
    println!("Please input a binary number");

    let mut bin = String::new();
    io::stdin()
        .read_line(&mut bin)
        .expect("Failed to read line");
    let bin = bin.trim();
    let converted = u32::from_str_radix(&bin, 2);
    println!("Result{:?}", converted);
}

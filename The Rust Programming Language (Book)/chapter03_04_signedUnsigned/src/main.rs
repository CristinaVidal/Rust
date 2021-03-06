#![allow(non_snake_case)]

use std::io;

fn main() {
    let mut number = String::new();

    println!("Number: ");

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read number !");

    println!("String: '{}'", number);
    let number: u32 = number.trim().parse::<u32>().unwrap();

    println!("u32 number: {}", number); 

}

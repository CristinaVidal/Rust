#![allow(non_snake_case)]

fn main() {
    println!("Integers ! :o\n");
    let a: i32 = 10;
    let b: i32 = 3;

    println!("{} / {} = {}", a, b, a / b);
    println!("________________________________\ninteger with floating-point ! :oo");

    let c: f32 = 10.0;
    let d: i32 = 3;

    println!("{} / {} = {}", c, d, c / d as f32);
}

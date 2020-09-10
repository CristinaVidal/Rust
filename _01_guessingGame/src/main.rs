#![allow(non_snake_case)]

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    let secretNumber = rand::thread_rng().gen_range(1, 101);
    println!("***Secret number: {}***", secretNumber);

    println!("Guess the number!");
    println!("Please input your guess:");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line !! :(");

        println!("Your guessed: {}", guess);

        let guess: u32 = guess.trim().parse().expect("This is not a number!");

        match guess.cmp(&secretNumber) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!")
        }
    }
}

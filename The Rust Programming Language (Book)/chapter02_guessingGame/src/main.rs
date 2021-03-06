#![allow(non_snake_case)]

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    let secretNumber = rand::thread_rng().gen_range(1, 101);

    println!("Guess the number!");
    loop {
        let mut guess = String::new();

        println!("Please input your guess:");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line !! :(");

        println!("Your guessed: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(newNumber) => newNumber,
            Err(ex) => {
                println!("Error: {}", ex);
                continue;
            }
        };
            
        
        match guess.cmp(&secretNumber) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}


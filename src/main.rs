use std::{io, cmp::Ordering};
use rand::Rng;
use std::error::Error;

mod guess;

use guess::Guess;

// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

fn main() -> Result<(), Box<dyn Error>> {
    println!("Guess the number!");

    let secret_number = rand::thread_rng()
                                .gen_range(1, 101);

    // println!("The secret number is {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)?;
        
        let guess: Guess = match guess.trim().parse() {
            Ok(num) => {
                Guess::new(num)
            },
            Err(_) => {
                panic!("Please enter a number");
            },
        };

        println!("You guessed: {}", &guess.value());

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("😕 Too small!"),
            Ordering::Greater => println!("😬 Too big!"),
            Ordering::Equal => {
                println!("🚀 You win!");
                break;
            }
        }
    }
    Ok(())
}
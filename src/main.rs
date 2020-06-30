use std::{io, cmp::Ordering};
use rand::Rng;
use std::error::Error;

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
        
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => {
                if num < 1 || num > 100 {
                    println!("Out of range, enter a number between 1 and 100");
                    continue;
                }
                num
            },
            Err(_) => {
                println!("Please enter a number");
                continue;
            },
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
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
use std::io;
use std::cmp::Ordering;
use rand::Rng; // The Rng trait defines methods that random number generators implement

// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng()
                                .gen_range(1, 101);

    println!("The secret number is {}", secret_number);

    println!("Please input your guess.");

    // Rust variables are immutable by default
    // The following example shows how to use 'mut' before the variable name to make a variable mutable:
    let mut guess = String::new();
    // The :: syntax in the ::new line indicates that new is an associated function of the String type
    // An associated function is implemented on a type, in this case String, rather than on a particular instance of a String. Some languages call this a static method.
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
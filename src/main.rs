use std::io;

// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // Rust variables are immutable by default
    // The following example shows how to use 'mut' before the variable name to make a variable mutable:
    let mut guess = String::new(); 
    // The :: syntax in the ::new line indicates that new is an associated function of the String type
    // An associated function is implemented on a type, in this case String, rather than on a particular instance of a String. Some languages call this a static method.
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", &guess);
}
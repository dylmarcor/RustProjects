extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number)

    println!("Please input your guess.");

    let mut guess = String::new(); // Get a mutable guess variable String

    io::stdin().read_line(&mut guess)   // Get user's input and add to guess var
        .expect("Failed to read line"); // & is a reference

    println!("You guess: {}", guess);
}

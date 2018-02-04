use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new(); // Get a mutable guess variable String

    io::stdin().read_line(&mut guess)   // Get user's input and add to guess var
        .expect("Failed to read line"); // & is a reference

    println!("You guess: {}") guess);
}

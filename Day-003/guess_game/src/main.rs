// Guess the number

use std::io;

fn main() {
    println!("- Guess the number -");
    println!("Input your guess: ");

    // Setting the var `guess` to a String, to start with.
    // This will accept any string (and treat numbers as strings)
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("\nYou guessed {}", guess);
}

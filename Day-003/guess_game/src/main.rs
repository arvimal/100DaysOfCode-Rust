// Guess the number

use std::io;

fn main() {
    println!("Guess the number: ");

    let mut guess: String = String::new();
    io::stdin().read_line(&mut guess).expect("Failed");
    println!("Your guess is {}", guess);
}

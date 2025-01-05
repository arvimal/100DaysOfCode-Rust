// Guessing a generated random number
use std::rand;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100); 
    println!("The secret number is {secret_number}");
}

// Generating and printing a random number 
use rand::Rng;

fn main() {
    println!("- Print a random number between 1 and 100 - ");
    
    let secret_number = rand::thread_rng().gen_range(1..100);
    println!("The random number is: {secret_number}");
}

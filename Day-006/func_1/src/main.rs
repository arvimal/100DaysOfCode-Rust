// Functions
// 1. Functions are defined by the `fn` keyword
// 2. Unlike Python, functions can be defined after it is referred
//
fn third_function() {
    println!("{}", 2 + 2);
}

fn main() {
    println!("Hello world!");
    second_function();
    third_function();
}

fn second_function() {
    println!("Second function!")
}

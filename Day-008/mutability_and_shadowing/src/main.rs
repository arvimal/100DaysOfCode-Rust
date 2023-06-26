fn main() {
    // By default, all vars in Rust are immutable.
    // There are two ways to change / update a variable

    // i. Mutability - By making the variable mutable with `mut` keyword
    println!("# Mutation");
    let mut first_number: i32 = 8;
    println!("First iteration of first_number: {}", first_number);
    first_number = 10; // The value is updated to 10 since the var is marked mutable
    println!("Second iteration of first_number: {}", first_number);

    // ii. Shadowing - Using `let` to over-ride the defined variable
    // Shadowing only works on variables that are in the same scope,
    // since moving across scopes drops variables
    println!("\n# Shadowing");
    let second_number: i32 = 100;
    println!("First iteration of second_number: {}", second_number);
    let second_number: i64 = 1000;
    println!("Second iteration of second_number: {}", second_number);
}

fn main() {
    // Shadowing can be used to over-write a variable
    // The difference between mutability and shadowing is
    // Mutation only allows to change the value with the same type
    // Hence, an i32 variable marked mutable can only be over-written by an i32
    //
    // Shadowing allows the same feature or over-writing the variable
    // But, the variable type can be over-ridden as well.
    let my_var: &str = "Hello, world";
    println!("Initial value of `my_var`: {my_var}");

    println!("Shadowing my_var with an integer: i32");
    let my_var: i32 = 100;
    println!("Value of `my_var` after shadowing: {}", my_var);
}

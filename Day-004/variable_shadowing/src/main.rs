// Variable shadowing
//
// Variable shadowing is done using the `let` keyword
// It is essentially over-writing the variable, hence the data type can be changed as well.
// Example: A string variable can be changed to a float or any other.
// On the contrary, a mutable variable cannot be type-changed, only the value can be changed.
// In both cases, the value is bound to the variable till the scope ends.
fn main() {
    let x = 100;
    println!("x is immutable, and the current value is {x}");
    let x = 10_000_000;
    println!("x is still immutable, but is shadowed by a new value using `let`. x is now {x}");
}

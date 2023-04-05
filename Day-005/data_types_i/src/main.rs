// Data types
//
fn main() {
    // Integers
    let x: u32 = 100_1000;
    let y: i128 = 110;
    let z = 2; // The compiler defaults to i32, if data-type is not specified.

    // Floating-points
    let a: f32 = 12.122;
    let b = 32.321; // The compiler defaults to f64, if data-type is not specified.
    
    println!("x: {x}, y: {y}, z = {z}, a = {a}, b = {b}");
}

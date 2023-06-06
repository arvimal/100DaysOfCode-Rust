// Arrays
// Various methods to initiate arrays
fn main() {
    // Direct assignment
    // `:?` has to be used to display the variable.
    // Error message: "error[E0277]: `[{integer}; 4]` doesn't implement `std::fmt::Display`"
    let array_1 = [1, 2, 3, 4];
    println!("array_1: {:?}", array_1);

    let months = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    println!("months: {:?}", months);

    // Initiating an empty array
    // Rust complains if `todo!()` macro is not listed, since the array is not initialized.
    // Rust also will complain about the `println!` line, since `a` is not initialized
    let _a: [i64; 10] = todo!();
    println!("Array `a`: {:?}", _a);
}

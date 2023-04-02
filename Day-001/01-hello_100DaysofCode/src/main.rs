// Rust Program
//
// 1. All rust programs has to have a main() function
// 2. println! is a built-in macro
// 3. All macros are called by appending `!` to the macro name.
// 4. Read more about Macros and Functions at https://doc.rust-lang.org/book/ch19-06-macros.html
// 5. Build the binary executable with `rustc <file-name>.rs`
//
fn main() {
    println!("Hello 100DaysofCode-Rust!");
    //
    /// {} can be used as place-holder for variables
    println!("A year has {} days", 365);
    //
    /// Named arguments can be used within {}
    println!("{subject} {verb} {object}", subject="The lazy ox", verb="is swimming", object="in the pond");

}

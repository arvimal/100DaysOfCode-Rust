// Constants
// Contants carry values that remain contstant across the program's life.
// Constants are defined with the `const` variable.
// 
// Constants are immutable, ALWAYS!!!
// Constants have to be type-defined, ALWAYS!!!
// Constants are defined in capitals with underscores, ALWAYS!!!

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    println!("Constant `THREE_HOURS_IN_SECONDS` = {THREE_HOURS_IN_SECONDS}");
}

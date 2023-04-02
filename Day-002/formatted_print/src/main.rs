// Day 002
// Formatted print - Printing formatted text
// 1. print(), println() etc are macros, rather than functions
// 2. A macro is in-built function that gets appended to the code
// by the compiler.
// 3. Macros are called with a `!` after the macro-name.
//
fn main() {
    println!("\n# Read the code to understand print formatting.\n");

    // I. Sting formatting
    println!("\n## I. String formatting.\n");
    // Print to stdout, without a new line appended
    print!("How are you?");
    // Print to stdout, with a new line appended
    println!("How are you today?");
    // Printing with placeholders / arguments
    println!("Hello {}, are you turning {} today?", "Alice", 32);
    // Printing with positional arguments 
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    // Printing with named arguments
    println!("{subject} {verb} {object}", verb="kicks", object="the ball", subject="The elephant");

    // II. Number formatting
    println!("\n## II. Number formatting.\n");
    println!("Base 10 of {}: {}", 100, 100);
    println!("Base 2 (Binary) of {}: {:b}", 100, 100);
    println!("Base 8 (Octal) of {}: {:o}", 100, 100);
    println!("Base 16 (Hexa-Decimal) of {}: {:x}", 100, 100);
    
    // III. Padding with spaces
    println!("\n## III. Padding.\n");
    println!("{number:>10} is greater than {number_1:>12}", number=10, number_1=1); 
    println!("The sum of {} and {} is: {:>5}", 1, 2, 3);

    // IV. Padding with numbers
    println!("\n## IV. Padding with numbers.\n");
    println!("Left side padding (Ex: pad with 0): {number:0>5}", number=1);
    println!("Right side padding: (Ex: pad with {number:#<6}", number=5);
}

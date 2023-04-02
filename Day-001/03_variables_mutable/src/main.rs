fn main() {
    // 1. First assignment works fine
    let a: i8 = 100;
    println!("a is {}", a);

    // 2. Re-assigning var `a` will not work, unless:
    //  2.1. Variable is marked mutable (let mut)
    //  2.2.  
    a = "Hello World!";
    println!(a);
}

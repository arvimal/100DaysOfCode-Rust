fn main() {
    // 1. First assignment works fine
    let a: i8 = 100;
    println!("a is {}", a);

    // 2. Re-assigning var `a` will not work, unless:
    // 2.1. Variable is not marked mutable (let mut)
    // 2.2. A different type (string) is assigned to an int type
    // a = "Hello World!";
    // println!(a);
}

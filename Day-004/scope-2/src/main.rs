fn main() {
    let x = 10;
    {
        let x = 100;
        println!("x is {x} in inner-scope");
    }
    println!("x is {x} in outer-scope")
}

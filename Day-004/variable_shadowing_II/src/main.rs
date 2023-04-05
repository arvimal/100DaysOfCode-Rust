// Variable scope and shadowing
//

fn main() {
    let x = 10;
    println!("Initial value of x: {x}");

    let x = x + 100;
    println!("Shadowed value of x: {x}");

    // Inner loop (A separate scope)
    {
        let x = x * 2;
        println!("Value of x, within the inner loop's scope: {x}");
        // Loop scope ends
    }

    // Outside the loop, back to main()'s scope
    println!("Value of x, outside the loop's scope: {x}");
}

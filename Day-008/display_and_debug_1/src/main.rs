fn main() {
    let doesnt_print = ();
    // Normal print - {} // This is the usual method, but certain object will not print with this
    // println!("{}", doesnt_print); // This will not print (std::fmt::Display)
    // Pretty print - {:#?}
    println!("Pretty print - {:#?}", doesnt_print); // This will print (Pretty print)
    // Debug print - {:?}
    println!("Debug print - {:?}", doesnt_print); // This will print (Debug print)
    // Do not print a new line 
    print!("Do not print a new line - {:#?}", doesnt_print);
    
}

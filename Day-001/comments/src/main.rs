fn main() {
    /// 1. This is a document string
    /// This is a doc-string used by Cargo
    /// This is a doc string on comments
    // 2. This is a single-line comment
    //
    let some_number = 10; // Everything after `//` is a comment
    let sec_number/*: i32 */ = 100;
    /* 3. This is a multi-line comment
     *   that spans more than one line
     *   and another
     */
    println!("{some_number} and {sec_number}");
}

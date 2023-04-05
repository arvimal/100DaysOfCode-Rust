// Data types - Builtins
// 
// Every value in Rust has a data-type.
// Rust is a statically-typed language, and it has to know all the data types at compile time
// In many cases, the compiled can figure out the data type, but it's encouraged to set it.
//
// Subsets of data-types
// 1. Scalar (Single value)
//  * Integer
//      - Signed [i8, i16, i32, i64, i128, isize]
//      - Unsigned [u8, u16, u32, u64, u128, usize]
//  * Floating-points
//      - Signed [f32, f64]
//  * Boolean (1 Byte)
//      - bool (True, False)
//  * Character 
//      - char
//
// 2. Compound (Multiple values) 
//  * Tuple
//      - Fixed in size / Immutable
//      - Holds multiple data types
//  * Array
//      - Fixed in size / Immutable
//      - Holds a single data type, ie. all elements have to be the same data-type.
//
//
// NOTE: 
// A `Vector` is similar to an Array, but can grow and shrink in size.
// A `Vector` is not a language builtin, but a Standard library implementation.

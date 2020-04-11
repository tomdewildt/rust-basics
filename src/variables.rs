// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn main() {
    // Define immutable variable
    let immutable = "Value 001";
    println!("Define immutable variable: {}", immutable);

    // Define mutable variable
    let mut mutable = 50;
    println!("Define mutable variable: {}", mutable);

    mutable = 100;
    println!("Update mutable variable: {}", mutable);

    // Define constant
    const CONSTANT: i32 = 001;
    println!("Define constant: {}", CONSTANT);

    // Assign multiple variables
    let (string1, string2) = ("Value 001", "Value 002");
    println!("Assign multiple variables: {} {}", string1, string2);
}

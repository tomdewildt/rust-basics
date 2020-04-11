// Primitive types
// Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
// Floats: f32, f64
// Booleans: bool
// Characters: char

// Rust is a statically type language, which means that it must know the types of all
// variables at compile time, however, the compiler can usually infer what type we
// want to use based on the value and how we use it.

pub fn main() {
    // Integer default is "i32"
    let integer = 1;
    println!("Integer default is i32: {}", integer);

    // Float default is "f64"
    let float = 2.5;
    println!("Float default is f64: {}", float);

    // Integer with explicit type
    let integer64: i64 = 45454545454545;
    println!("Integer with explicit type: {}", integer64);

    // Find max size
    println!("Find max size of i32: {}", std::i32::MAX);
    println!("Find max size of i64: {}", std::i64::MAX);

    // Boolean
    let boolean = true;
    println!("Boolean: {}", boolean);

    // Get boolean from expression
    let expression = 10 > 5;
    println!("Get boolean from expression: {}", expression);

    // Character
    let character = 'a';
    let unicode = '\u{1F600}';
    println!("Character: {}", character);
    println!("Unicode character: {}", unicode);
}

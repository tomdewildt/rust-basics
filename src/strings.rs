// Str is a immutable fixed length string stored in memory
// String is a growable, heap-allocated data structure, use this when you need to modify or own string data

pub fn main() {
    // Define immutable string
    let string = "Hello";
    println!("Define immutable string: {}", string);

    // Get string length
    println!("Get string length: {}", string.len());

    // Define growable string
    let mut string = String::from("Hello ");
    println!("Define growable string: {}", string);

    // Append char
    string.push('W');
    println!("Append char: {}", string);

    // Append string
    string.push_str("orld");
    println!("Append string: {}", string);

    // Get capacity (in bytes)
    println!("Get capacity (in bytes): {}", string.capacity());

    // Is string empty
    println!("Is string empty: {}", string.is_empty());

    // String contains word
    println!("String contains word: {}", string.contains("World"));

    // Replace word in string
    println!(
        "Replace word in string: {}",
        string.replace("World", "There")
    );

    // Loop through string by whitespace
    for (index, word) in string.split_whitespace().enumerate() {
        println!("Loop through string by whitespace ({}): {}", index, word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("Create string with capacity: {}", s);

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
}

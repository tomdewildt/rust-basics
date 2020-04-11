// Print is used to print text to stdout

pub fn main() {
    // Print string
    println!("Print string: Test");

    // Basic formatting
    println!("Basic formatting: {}", 1);

    // Positional arguments
    println!(
        "Positional arguments: {0} {1} {2} {0}",
        "Value 001", "Value 002", "Value 003"
    );

    // Named arguments
    println!(
        "Named arguments: {key}: {value}",
        key = "Key",
        value = "Value"
    );

    // Placeholder traits
    println!(
        "Placeholder traits: binary:{:b} hex:{:x} octal:{:o}",
        10, 10, 10
    );

    // Placeholder for debug traits
    println!("Placeholder for debug traits: {:?}", (12, true, "hello"));

    // Basic math
    println!("Basic math: {}", 10 + 10);
}

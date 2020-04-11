// Tuples group together different values of different types
// Maximum of 12 elements

pub fn main() {
    // Define tuple
    let tuple: (&str, &str, i8) = ("Value 001", "Value 002", 50);
    println!("Define tuple: {} {} {}", tuple.0, tuple.1, tuple.2);
}

// Conditionals are used to check the condition of something and act on the result

pub fn main() {
    let integer = 50;

    // If else statement
    if integer >= 50 {
        println!("If else statement: value is greater than or equal to 50");
    } else {
        println!("If else statement: value is less than 50");
    }

    // One line if else statement
    let expression = if integer >= 50 { true } else { false };
    println!("One line if else statement: {}", expression);
}

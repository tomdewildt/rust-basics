// Loops are used to iterate until a condition is met

pub fn main() {
    let mut integer = 0;

    // Infinite loop
    loop {
        integer += 1;
        println!("Infinite loop: {}", integer);

        if integer >= 5 {
            break;
        }
    }

    // While loop
    while integer <= 10 {
        println!("While loop: {}", integer);
        integer += 1;
    }

    // For range loop
    for value in 0..5 {
        println!("For range loop: {}", value);
    }
}

use std::env;

pub fn main() {
    // Collect CLI args
    let args: Vec<String> = env::args().collect();
    let command = args[0].clone();

    println!("Collect CLI command: {}", command);
    println!("Collect CLI args: {:?}", &args[1..]);
}

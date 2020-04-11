mod arrays;
mod cli;
mod conditionals;
mod enums;
mod functions;
mod loops;
mod pointers;
mod print;
mod strings;
mod structs;
mod tuples;
mod types;

fn main() {
    println!("--- Arrays ---");
    arrays::main();
    println!("");

    println!("--- Cli ---");
    cli::main();
    println!("");

    println!("--- Conditionals ---");
    conditionals::main();
    println!("");

    println!("--- Enums ---");
    enums::main();
    println!("");

    println!("--- Functions ---");
    functions::main();
    println!("");

    println!("--- Loops ---");
    loops::main();
    println!("");

    println!("--- Pointers ---");
    pointers::main();
    println!("");

    println!("--- Print ---");
    print::main();
    println!("");

    println!("--- Strings ---");
    strings::main();
    println!("");

    println!("--- Structs ---");
    structs::main();
    println!("");

    println!("--- Tuples ---");
    tuples::main();
    println!("");

    println!("--- Types ---");
    types::main();
    println!("");
}

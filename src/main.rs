mod arrays;
mod cli;
mod conditionals;
mod enums;
mod functions;
mod loops;
mod pointers;

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
}

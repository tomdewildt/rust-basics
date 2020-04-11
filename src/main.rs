mod arrays;
mod cli;
mod conditionals;

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
}

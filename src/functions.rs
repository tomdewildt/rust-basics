pub fn main() {
    // Function without return
    function_without_return("Hello ", "World");

    // Function with return
    println!("Function with return: {}", function_with_return(25, 25));

    // Inline function
    let inline_function = |number1: i32, number2: i32| number1 + number2;
    println!("Inline function: {}", inline_function(50, 50));
}

fn function_without_return(string1: &str, string2: &str) {
    println!("Function without return: {} {}", string1, string2);
}

fn function_with_return(number1: i32, number2: i32) -> i32 {
    number1 + number2
}

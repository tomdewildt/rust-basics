// Vectors are resizeable arrays

pub fn main() {
    let mut vector: Vec<i32> = vec![1, 2, 3, 4];

    // Reassign value
    vector[2] = 20;
    println!("Reassign value: {:?}", vector);

    // Add values to vector
    vector.push(5);
    vector.push(6);
    println!("Add values to vector: {:?}", vector);

    // Pop off last value
    vector.pop();
    println!("Pop off last number: {:?}", vector);

    // Get single value
    println!("Get single value: {}", vector[2]);

    // Get vector length
    println!("Get vector length: {}", vector.len());

    // Loop through vector values
    for (index, value) in vector.iter().enumerate() {
        println!("Loop through vector values ({}): {}", index, value);
    }

    // Loop & mutate values
    for value in vector.iter_mut() {
        *value *= 2;
    }
    println!("Loop & mutate values: {:?}", vector);
}

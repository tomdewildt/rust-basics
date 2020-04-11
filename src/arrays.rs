// Arrays fixed list where elements are same data type

use std::mem;

pub fn main() {
    // Define array
    let mut array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Define array: {:?}", array);

    // Get single value
    println!("Get single value: {}", array[0]);

    // Reassign a value
    array[2] = 20;
    println!("Reassign a value: {:?}", array);

    // Get array length
    println!("Get array length: {}", array.len());

    // Get array size in memory
    println!("Get array size in memory: {}", mem::size_of_val(&array));

    // Get slice
    let slice: &[i32] = &array[1..3];
    println!("Get slice: {:?}", slice);
}

// Pointers point to a resource in memory
// With non-primitives, if you assign another variable to a piece of data
// the first variable, will no longer hold that value. You'll need to use a
// reference (&) to point to the resource.

pub fn main() {
    // Primitive array
    let array1 = [1, 2, 3];
    let array2 = array1;
    println!("Primitive array: {:?}", (array1, array2));

    // Non-primitive vector
    let vector1 = vec![1, 2, 3];
    let vector2 = &vector1;
    println!("Non-primitive vector: {:?}", (&vector1, vector2));
}

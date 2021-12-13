use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("First element of slice: {}", slice[0]);
    println!("The slice has {} elements.", slice.len());
}

fn main() {
    // Fixed size array
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be intialized to same value
    // let ys: [i32; 500] = [0; 500];

    // Indexing starts at 0
    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);

    // len() returns count of elements in array
    println!("Number/Length of array xs: {}", xs.len());

    // Arrays are stack allocated
    println!("Array occupies bytes: {}", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices
    analyze_slice(&xs);

    // Slices can point to section of array
    // They are of the form [starting_index .. ending_index]
    // starting_index is first position in the slice
    // ending_index is one more than the last position in the slice
    println!("borrow a section of the array as slice");
    analyze_slice(&xs[1 .. 4]);
}
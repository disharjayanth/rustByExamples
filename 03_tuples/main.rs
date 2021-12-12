// Tuples can be used as function arguments and return values
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind members of tuple to variable
    let (integer, boolean) = pair;

    (boolean, integer)
}

// Following struct is for activity
#[derive(Debug)]
struct Matrix (f32, f32, f32, f32);

fn main() {
    // Tuple with buch of different values
    let long_tuple = (1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true);

    println!("All values in a tupe: {:?}",long_tuple);
    println!("Long tuple with first value: {}", long_tuple.0);
    println!("Long tuple with second value: {}", long_tuple.1);

    // Tuples of tuple
    let tuples_of_tuple = ((1u8, 2u16, 2u32), (4u64, -1i8));
    println!("Tuples of tuple: {:?}", tuples_of_tuple);

    // Long tuples cannot be printed
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18);
    // println!("Too long tuple: {:?}", too_long_tuple);

    let pair = (1, true);
    println!("Pair: {:?}", pair);

    // Calling reverse function to reverse given pair
    println!("Reverse of pair: {:?}", reverse(pair));

    // To create one element tuples , comma is required to tell them apart
    // from a literal surrounded by parenthesis
    println!("One element tuple: {:?}", (5u32,));
    println!("Just an integer: {:?}", (5u32));

    // tuples can be destructed to create bindings
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?} {:?} {:?} {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 1.3, 1.4);
    println!("{:?}", matrix);
}


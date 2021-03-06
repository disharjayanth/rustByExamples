fn main() {
    // Integer addition
    println!("1 + 2 = {}", 1i32 + 2);

    // Integer substraction
    println!("1 - 2 = {}", 1i32 - 2);

    // Short-circuiting boolean logic
    println!("true AND false is: {}", true && false);
    println!("true OR false is: {}", true || false);
    println!("NOT true is: {}", !true);

    // Bitwise operation
    println!("0011 AND 0101 is: {:04b}", 0b0011 & 0b0101);
    println!("0011 OR 0101 is: {:04b}", 0b0011 | 0b0101);
    println!("0011 XOR 0101 is: {:04b}", 0b0011 ^ 0b0101);
    println!("1 << 5 is: {}", 1 << 5);
    println!("0x88 >> 2 is: 0x{:x}", 0x80 >> 2);

    // Use _ to improve readability
    println!("One million is written as: {}", 1_000_000u32);
}
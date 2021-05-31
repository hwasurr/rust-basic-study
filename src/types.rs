pub fn run() {
    // primitive types -
    // Integers
    // Floats
    // Boolean (bool)
    // Characters (char) not string
    // Tuples
    // Arrays

    // Default is "i32"
    let x = 1;

    // Default is "f64"
    let y = 2.5;

    // Add explicit type
    let z: i64 = 454545454545454545;

    // Find max size
    println!("Max i32 {}", std::i32::MAX);
    println!("Max i64 {}", std::i64::MAX);

    // Boolean
    let is_active = true;

    // Get boolean from expression
    let is_greater: bool = 10 > 0;

    println!("{:?}", (x, y, z, is_active, is_greater));

    // Char
    let a1 = 'a'; // you can't 'ab'
    let face = '\u{1F600}';
    println!("{:?}", (a1, face));
}

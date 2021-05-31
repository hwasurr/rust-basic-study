pub fn run() {
    // there is two types in string
    // * Primitive str -> Immutable  fixed-length string somewhere in memory
    // * String -> Growable heap-allocated data structure:: use when you need to modify or own string data

    let hello1 = "Hello "; // Primitive str
    println!("{}", hello1);

    let mut hello = String::from("Hello "); // String
    println!("{}", hello);

    // Get length of strings
    println!("length : {}", hello.len());

    hello.push('W'); // Add char to String
    hello.push_str("orld"); // Add strings

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());
    // Check empty
    println!("Is Empty: {}", hello.is_empty());
    // Contains
    println!("Contains 'World' {}", hello.contains("World"));
    // Replace
    println!("Replace: {}", hello.replace("World", "There"));
    // Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing (왼쪽과 오른쪽이 동일하지 않으면 프로그램종료)
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);
}

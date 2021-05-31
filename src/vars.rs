pub fn run() {
    // variables are immutable by default
    // Rust is a block scoped language

    let name = "Brad";
    let mut age = 28;

    println!("My name is {} and i am {}", name, age);
    age = 29;
    println!("My name is {} and i am {}", name, age);

    // Define constants
    const ID: i32 = 001; // using const, have to define explicitly
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Brad", 37);
    println!("{} is {}", my_name, my_age);
}

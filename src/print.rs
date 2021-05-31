pub fn run() {
    // Print to console
    println!("hello from the print.rs file");

    println!("{}", 1);

    // basic formatting
    println!("{} is from {}", "hwasurr", "house");

    // positional arguments
    println!("{0} is from {1} and teamlead of {1} ", "hwasurr", "onad");
    // named argument
    println!(
        "{name} likes to play {activity}",
        name = "hwasurr",
        activity = "baseball"
    );
    // placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}

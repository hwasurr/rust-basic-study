use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();

    let command = args[1].clone();

    let name = "Brad";
    
    println!("Command: {}", command);

    if command == "test" {
        println!("Hi {}, how are you", name);
    } else if command == "status" {
        println!("Status : 100%");
    } else {
        println!("invalid command : {}", command );
    }
}
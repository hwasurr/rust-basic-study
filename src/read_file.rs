use std::fs::File;
use std::io::prelude::*;

pub fn run() {
    let mut file = File::open("src/data/info.txt")
        .expect("can't open the file!");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Oops! cant not read the file...");

    println!("File Contents\n\n{}", contents);
}
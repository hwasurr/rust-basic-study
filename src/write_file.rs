use std::fs::File;
use std::io::prelude::*;

pub fn run() {
    let folder = std::fs::read_dir("src/output");
    if folder.is_err() {
        std::fs::create_dir("src/output")
            .expect("cannot create directory");
    }

    let mut file = File::create("src/output/output.txt")
        .expect("Could not create file!");

    file.write_all(b"Welcome to Rust!")
        .expect("Cannot write to file sorry.");
}
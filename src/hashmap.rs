use std::collections::HashMap;

pub fn run() {
    let mut marks = HashMap::new();
    // Add values;
    marks.insert("Rust Programming", 96); // key, value
    marks.insert("Web Dev", 94); // key, value
    marks.insert("UX Design", 75); // key, value
    marks.insert("Professional Computing Studies", 100); // key, value

    // Find length of hashmap
    println!("How many subject have you studied ? {}", marks.len());

    // Get a single value
    match marks.get("Web Dev") {
        Some(mark) => println!("you got {} for Web dev", mark),
        None => println!("you didn't study Web dev"),
    }

    // Remove a single value
    marks.remove("UX Design");
    marks.remove("invaild key"); // do nothing

    // Loop through HashMap
    for (key, value) in &marks {
        println!("For {} you got {}%", key, value);
    }

    // Check for value
    println!(
        "did you study C++? {}",
        marks.contains_key("C++ Programming")
    )
}

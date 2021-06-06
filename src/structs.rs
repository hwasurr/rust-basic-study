// Traditional Struct
struct Color {
    red: u8, // u8 = 0-255
    green: u8,
    blue: u8,
}

// Tuple struct
struct TupleColor(u8,u8,u8,);

struct Person {
    first_name: String,
    last_name: String,
}

// implement methods to struct
impl Person {
    // Construct person
    fn new(first: &str, last:&str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    // Get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    // Structs - used to create custom data types

    let mut c = Color {
        red: 255, green: 0, blue: 0
    };

    c.red = 200;

    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut c = TupleColor(255, 0, 0);
    c.1 = 10;
    println!("TupleColor: {} {} {}", c.0, c.1, c.2);

    let mut p = Person::new("kang", "hwasoo");
    println!("Person: {} {}", p.first_name, p.last_name);
    p.set_last_name("williams");
    println!("Person: {}", p.full_name());
    println!("Person Tuple: {:?}", p.to_tuple());
}
struct Person {
    name: String,
    age: u8,
}

trait HasVoiceBox {
    // Speak
    fn speak(&self);
    // Check if can speak
    fn can_speak(&self) -> bool;
}

impl ToString for Person {
    fn to_string(&self) -> String {
        return format!("My name is {} and I am {}", self.name, self.age);
    }
}

impl HasVoiceBox for Person {
    fn speak(&self) {
        println!("Hello my name is {}", self.name);
    }

    fn can_speak(&self) -> bool {
        if self.age > 0 {
            return true;
        }
        return false;
    }
}

pub fn run() {
    let me = Person { name: String::from("hwasoo"), age: 28 };

    println!("{}", me.to_string());
    println!("{} can speak ? {}", me.name, me.can_speak())
}
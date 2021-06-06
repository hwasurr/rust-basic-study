
enum Movement {
    // Variants
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(m: Movement) {
    // match 는 switch 라고 생각하면 된다.

    // Perform action depending on info
    match m {
        Movement::Up => println!("Avatar moving Up"),
        Movement::Down => println!("Avatar moving Down"),
        Movement::Left => println!("Avatar moving Left"),
        Movement::Right => println!("Avatar moving Right"),
        // 10|11 => println!("or operator in match statement")
        // 12..20 => println!("12~20 -> this statement")
        _ => panic!("Movement doesn't matched"),
    }
}

pub fn run() {
    // Enums are type which have a few definite values

    let avatar1 = Movement::Left;
    let avatar2 = Movement::Up;
    let avatar3 = Movement::Right;
    let avatar4 = Movement::Down;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}
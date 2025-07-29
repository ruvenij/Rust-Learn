#[derive(Debug)]
enum Movement {
    Up,
    Down,
    Left,
    Right,
}

enum Location {
    Unknown,
    Anonymous,
    Known(f64, f64)
}

impl Location {
    fn display(&self) {
        match self {
            Location::Unknown => println!("Location is unknown"),
            Location::Anonymous => println!("Location is anonymous"),
            Location::Known(x, y,) => println!("Location is {} and {}", x, y)
        }
    }
}

fn move_avatar(m: Movement) {
    match m {
        Movement::Up => println!("Avatar moving up"),
        Movement::Down => println!("Avatar moving down"),
        Movement::Left => println!("Avatar moving left"),
        Movement::Right => println!("Avatar moving right")
    }
}

pub fn run() {
    move_avatar(Movement::Down);
    move_avatar(Movement::Up);
    move_avatar(Movement::Right);
    move_avatar(Movement::Left);

    let countdown = [5, 4, 3, 2, 1];
    let number = countdown.get(5);
    println!("Result for index 5 : {:?}", number);

    let number = number.unwrap_or(&0) + 1;
    println!("Unwrap Result for index 5 : {:?}", number);

    let number = countdown.get(1);
    let number = match number {
        Some(number) => number + 1,
        None => 0,
    };
    println!("Number is : {:?}", number);

    let my_num = Some(13);
    if let Some(13) = my_num {
        println!("Number is 13");
    }

    let address = Location::Unknown;
    address.display();
    let address = Location::Anonymous;
    address.display();
    let address = Location::Known(28.64736, (-80.6734));
    address.display();
}

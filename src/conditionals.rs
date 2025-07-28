pub fn run() {
    let age = 12;
    let check_id = false;
    if age >= 18 && check_id {
        println!("You are an adult!");
    } else if age < 18 && check_id {
        println!("You are a minor!");
    } else {
        println!("Age is not needed!");
    }

    // shorthand if
    let is_of_age = if age >= 18 { true } else { false };
    println!("Is of age: {}", is_of_age);
}

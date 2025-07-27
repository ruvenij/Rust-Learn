pub fn run() {
    let name = "Ruv";
    println!("My name is {}", name);

    let mut age = 30;
    println!("My age is {}", age);
    age = 31;
    println!("Next year, I will be {}", age);

    const ID: i32 = 001;
    println!("My ID is {}", ID);

    let (my_name, my_age) = ("Ruv", 30);
    println!("{} is {}", my_name, my_age);
}
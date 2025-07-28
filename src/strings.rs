pub fn run() {
    let hello = "Hello";
    println!("{} {}", hello, "world");

    let mut world = String::from("World ");
    println!("{} {} {}", world, world.len(), world.capacity());

    world.push('w');
    println!("{} {} {}", world, world.len(), world.capacity());

    world.push_str("orld");
    println!("{} {} {}", world, world.len(), world.capacity());

    // check if empty
    if !world.is_empty() {
        println!("World is not empty");
    }

    // contains
    if world.contains("world") {
        println!("Word contains world");
    }

    // replace
    world = world.replace("world", "hello");
    println!("{}", world);

    // iterate after split
    for word in world.split_whitespace() {
        println!("{}", word);
    }

    // assertion
    let new_text = "AB";
    assert_eq!(2, new_text.len());
}

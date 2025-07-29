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

    let message = String::from("Hello, world!!!");
    let world_slice = &message[7..12];
    println!("Slice from message {}", world_slice);

    let mut sentence = String::from("Greetings from earth!");
    let mut first_word = get_first_word(&sentence);
    println!("First word is: {}", first_word);

    sentence = String::from("Hello!");
    first_word = get_first_word(&sentence);
    println!("First word is: {}", first_word);

    test_trim_spaces();
}

fn get_first_word(input: &String) -> &str {
    for (index, character) in input.char_indices() {
        if character == ' ' {
            println!("A space found at index {}", index);
            return &input[..index];
        }
    }

    return &input;
}

fn test_trim_spaces() {
    let mut result: &str;
    let test1 = "We need more space.";
    result = trim_spaces(&test1);
    println!("Result is {}", result);
    assert_eq!(result, "We need more space.");

    let test2 = String::from("   There's space in front.");
    result = trim_spaces(&test2);
    println!("Result is {}", result);
    assert_eq!(result, "There's space in front.");

    let test3 = String::from("There's space to the rear. ");
    result = trim_spaces(&test3[..]);
    println!("Result is {}", result);
    assert_eq!(result, "There's space to the rear.");

    let test4 = "  We're surrounded by space!    ";
    result = trim_spaces(test4);
    println!("Result is {}", result);
    assert_eq!(result, "We're surrounded by space!");

    let test5 = "     ";
    result = trim_spaces(test5);
    println!("Result is {}", result);
    assert_eq!(result, "");

    let test6 = "";
    result = trim_spaces(test6);
    println!("Result is {}", result);
    assert_eq!(result, "");

    let test7 = " 🚀 ";
    result = trim_spaces(test7);
    println!("Result is {}", result);
    assert_eq!(result, "🚀");
    println!("Tests passed!");
}

fn trim_spaces(input: &str) -> &str {
    if input.len() == 0 {
        return "";
    }

    let mut start_index = 0;
    let mut end_index = input.len() - 1;
    for (index, character) in input.char_indices() {
        println!(
            " Start Index {} {} Character {}",
            index, start_index, character
        );
        if character == ' ' {
            if index == input.len() - 1 {
                break;
            }

            start_index = index + 1;
        } else {
            break;
        }
    }

    for (index, character) in input.char_indices().rev() {
        println!(" End Index {} {} Character {}", index, end_index, character);
        if character == ' ' {
            if index == 0 {
                break;
            }

            end_index = index - 1;
        } else {
            break;
        }
    }

    if end_index > start_index {
        return &input[start_index..end_index + 1];
    }

    return "";
}

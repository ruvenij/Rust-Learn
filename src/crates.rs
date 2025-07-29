use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn guess_number() {
    let number = rand::rng().random_range(1..100);

    loop {
        let mut buffer = String::new();
        println!("Enter your guess (1-100): ");
        let _ = io::stdin().read_line(&mut buffer);

        let guess: u8 = buffer.trim().parse().unwrap();
        match guess.cmp(&number) {
            Ordering::Less => println!("Guessed value is low"),
            Ordering::Greater => println!("Guessed value is high"),
            Ordering::Equal => 
            {
                println!("You guessed the correct value, Secret number was {}", number);
                break;
            },
        }
    }
}

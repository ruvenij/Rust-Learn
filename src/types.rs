pub fn run() {
    let x = 1;
    let y = 2.5;
    let z = 'a';
    let is_active = true;
    let name = "Ruv";   
    let tuple: (i32, f64, char) = (1, 2.5, 'a');
    let array: [i32; 3] = [1, 2, 3];
    let q:i64 = 10000000000;


    println!("Max of i32 is {}", std::i32::MAX);
    println!("{} {} {} {} {} {}", x, y, z, is_active, name, q);
    println!("Second element of the tuple is {}", tuple.1);
    println!("First element of the array is {}", array[0]);

    #[derive(Debug)]
    struct Person{
        name: String,
        age: u8,
    }

    let p = Person{name: "Ruv".to_string(), age: 30};
    println!("Person's name is {} and age is {}", p.name, p.age);
    println!("{:?}", p);
    println!("{} is {} now", p.name, p.age);

}
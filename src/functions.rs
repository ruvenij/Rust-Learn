pub fn run() {
    greeting("Hello", "James");

    let sum = add(1, 2);
    println!("Sum is {}", sum);

    // closures
    let n3 = 5;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Closure sum is {}", add_nums(3, 4));

    let celcius = 24.0;
    let farenheit = celsius_to_farenheit(celcius);
    println!("{}°C is {}°F", celcius, farenheit);
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, Nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    //return n1 + n2;
    n1 + n2 // implicit return
}

fn celsius_to_farenheit(celcius: f64) -> f64 {
    (1.8 * celcius) + 32.0
}

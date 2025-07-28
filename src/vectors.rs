pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    println!("{:?}", numbers);
    println!("Second element is {}", numbers[1]);

    numbers[1] = 20;
    println!("Vector after change is {:?}", numbers);

    numbers.push(11);
    numbers.push(12);
    println!("Vector after pushing elements is {:?}", numbers);

    let element = numbers.pop();
    println!("Popped element is {:?}", element);
    println!("Vector after popping is {:?}", numbers);

    // loop through vector
    for x in numbers.iter() {
        println!("Number is {}", x);
    }

    // loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("New vectors after changing {:?}", numbers);
}

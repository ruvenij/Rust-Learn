pub fn run() {
    println!("Hello from the print file");

    println!("Number: {}", 1);
    println!("Hello, world! {}", true);
    println!("derere {}", true);
    println!("{} {}", 1.3444, 5.6774);
    println!("{0} {1} {0}", 1, 2);

    // named params
    println!("{name} likes {activity}", name = "Ruv", activity = "Rust");

    // placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // basic math
    println!("10 + 10 = {}", 10 + 10);
}

pub fn run() {
    let person: (&str, &str, i8) = ("Ruv", "Developer", 34);
    println!(
        "{} is a {} and is {} years old",
        person.0, person.1, person.2
    );
}

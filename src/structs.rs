struct Colour {
    red: u8,
    green: u8,
    blue: u8,
}

// tuple struct
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u32,
}

impl Person {
    // construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
            age: 30,
        }
    }

    fn print(&self) {
        println!("Person is {} {} {}", self.first_name, self.last_name, self.age);
    }

    fn get_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    fn to_tuple(&self) -> (String, String) {
        (self.first_name.clone(), self.last_name.clone())
    }
}

struct Rectangle {
    width: f64,
    height: f64
}

impl Rectangle {
    fn new(width: f64, height: f64) -> Rectangle{
        Rectangle {
            width: width,
            height: height,
        }
    }

    fn get_area(&self) -> f64 {
        self.width * self.height
    }

    fn scale(&mut self, input: f64) {
        self.height *= input;
        self.width *= input;
    }
}

pub fn run() {
    let mut c = Colour {
        red: 255,
        green: 0,
        blue: 0,
    };

    println!("Colour {} {} {}", c.red, c.green, c.blue);

    c.red = 200;
    println!("Colour {} {} {}", c.red, c.green, c.blue);

    // tuple struct
    let p1 = Point(10, 20, 30);
    println!("Point coordinates: ({}, {}, {})", p1.0, p1.1, p1.2);

    let mut per1 = Person::new("John", "Doe");
    per1.print();
    println!("{}", per1.get_name());

    per1.set_last_name("Smith");
    per1.print();

    let per_tuple = per1.to_tuple();
    println!("Tuple representation: {:?}", per_tuple);

    let per2 = Person {
        first_name: String:: from("Sherlock"),
        last_name: String::from("Holmes"),
        ..per1 // assign rest of the values from per1
    };

    println!("Person 2 values : {:?}", per2);

    let mut rect = Rectangle::new(1.2, 3.4);
    assert_eq!(rect.get_area(), 4.08);
    rect.scale(0.5);
    assert_eq!(rect.get_area(), 1.02);
    println!("Rectangle area after scaling: {}", rect.get_area());
}

struct Colour {
    red: u8,
    green: u8,
    blue: u8,
}

// tuple struct
struct Point(i32, i32, i32);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    fn print(&self) {
        println!("Person is {} {}", self.first_name, self.last_name);
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
}

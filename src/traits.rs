use std::fmt;
struct Satellite {
    name: String,
    velocity: f64
}

impl fmt::Display for Satellite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
       write!(f, "Satellite {} is travelling at {}", self.name, self.velocity)
    }
}

pub fn run() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.67
    };

    println!("Telescope details: {}", hubble);
}
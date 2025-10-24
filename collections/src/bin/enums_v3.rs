use std::fmt;

enum Animal {
    Dog(String),
    Cat(String),
    Bird(String),
}

impl fmt::Display for Animal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Animal::Dog(name) => write!(f, "Dog named {}", name),
            Animal::Cat(name) => write!(f, "Cat named {}", name),
            Animal::Bird(name) => write!(f, "Bird named {}", name),
        }
    }
}

fn animal_sound(animal: &Animal) -> &'static str {
    match animal {
        Animal::Dog(_) => "Bark",
        Animal::Cat(_) => "Meow",
        Animal::Bird(_) => "Tweet",
    }
}
fn main() {
    let pet1 = Animal::Dog(String::from("Buddy"));
    let pet2 = Animal::Cat(String::from("Whiskers"));
    let pet3 = Animal::Bird(String::from("Tweety"));

    println!("{} which makes sound of: {}", &pet1, animal_sound(&pet1));
    println!("{} which makes sound of: {}", &pet2, animal_sound(&pet2));
    println!("{} which makes sound of: {}", &pet3, animal_sound(&pet3));
}

// struct Point {
//     x: i32,
//     y: i32,
// }
// fn main() {
//     let x = 5;
//     let y = 10;
//     let p = Point { x, y };
//     println!("Point: ({}, {})", p.x, p.y);
// }

// struct Book {
//     title: String,
//     pages: u32,
// }
// impl Book {
//     fn reading_time(&self) -> u32 {
//         self.pages / 2
//     }
// }
// fn main() {
//     let book = Book {
//         title: String::from("Rust Programming"),
//         pages: 500,
//     };
//     println!("Reading time: {} minutes", book.reading_time());
// }

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
// fn main() {
//     let msg = Message::Move { x: 10, y: 20 };
//     match msg {
//         Message::Quit => println!("Quit"),
//         Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
//         Message::Write(text) => println!("Text message: {}", text),
//         Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
//     }
// }

// struct Rectangle {
//     width: u32,
//     length: u32
// }
// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         length: 50,
//     };
//     let rect2 = rect1;
//     println!("Rectangle: {}x{}", rect2.width, rect2.length);
// }

// use std::fmt::{self, Display};
// struct Point {
//     x: i32,
//     y: i32,
// }
// impl fmt::Display for Point {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Point({}, {})", self.x, self.y)
//     }
// }
// fn main() {
//     let p = Point { x: 5, y: 10 };
//     println!("{}", p);
// }

// fn main() {
//     let mut v = vec![1, 2, 3, 4];
//     for i in &mut v {
//         *i += 10;
//     }
//     println!("{:?}", v); // Output: [11, 12, 13, 14]
// }

// use std::collections::HashMap;
// fn main() {
//     let mut scores = HashMap::new();
//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Red"), 20);
//     let team_name = String::from("Blue");
//     let score = scores.get(&team_name);
//     match score {
//         Some(&score) => println!("Score: {}", score),
//         None => println!("Team not found"),
//     }
// }

// use std::collections::HashMap;
// fn main() {
//     let mut scores = HashMap::new();
//     scores.insert(String::from("Blue"), 10);
//     scores.entry(String::from("Blue")).and_modify(|e| *e += 50).or_insert(50);
//     scores.entry(String::from("Green")).or_insert(50);
//     println!("{:?}", scores); // Output: {"Blue": 60, "Green": 50}
// }

// enum TrafficLight {
//     Red,
//     Yellow,
//     Green,
// }
// impl TrafficLight {
//     fn duration(&self) -> u8 {
//         match self {
//             TrafficLight::Red => 60,
//             TrafficLight::Yellow => 5,
//             TrafficLight::Green => 30,
//         }
//     }
// }
// fn main() {
//     let light = TrafficLight::Green;
//     println!("Green light duration: {} seconds", light.duration());
// }

// fn main() {
//     let numbers = vec![1, 2, 3];
//     println!("Fourth element: {}", numbers[3]); // This will panic
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
// impl Default for Rectangle {
//     fn default() -> Self {
//         Rectangle {
//             width: 30,
//             height: 50,
//         }
//     }
// }
// fn main() {
//     let rect: Rectangle = Default::default();
//     println!("{:?}", rect); // Output: Rectangle { width: 30, height: 50 }
// }

// fn main() {
//     let v1 = vec![1, 2, 3];
//     let v2 = v1;
//     println!("{:?}", v2);
// }

// use std::env;
// fn main() -> Result<(), String> {
//     let var_value = env::var("NUMBER").map_err(|_| "Env var not found".to_string())?;
//     let num: i32 = var_value.parse().map_err(|_| "Failed to parse as number".to_string())?;
//     println!("Value: {}", num);
//     Ok(())
// }

// enum Data {
//     Integer(i32),
//     Float(f64),
//     Text(String),
// }
// fn main() {
//     let mut data = vec![
//         Data::Integer(42),
//         Data::Float(3.14),
//         Data::Text(String::from("Hello")),
//     ];
//     for item in data {
//         match item {
//             Data::Integer(i) => println!("Integer: {}", i),
//             Data::Float(f) => println!("Float: {}", f),
//             Data::Text(s) => println!("Text: {}", s),
//         }
//     }
// }

struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn increase_width(&mut self, increment: u32) {
        self.width += increment;
    }
}
fn main() {
    let mut rect = Rectangle {
        width: 30,
        height: 50,
    };
    rect.increase_width(10);
    println!("Updated width: {}", rect.width);
}

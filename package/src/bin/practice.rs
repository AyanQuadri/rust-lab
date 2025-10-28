// mod calculator {
//     pub fn add(a: i32, b: i32) -> i32 {
//         a + b
//     }
//     pub fn subtract(a: i32, b: i32) -> i32 {
//         a - b
//     }
// }
// fn main() {
//     let sum = crate::calculator::add(5, 3);
//     let difference = crate::calculator::subtract(5, 3);
//     println!("Sum: {}, Difference: {}", sum, difference);
// }

// use rand::Rng;
// fn main() {
//     let mut rng = rand::rng();
//     let random_number: i32 = rng.random_range(1..11);
//     println!("Random Number: {}", random_number);
// }

pub mod math;

fn main() {
    let result = math::advanced::square(4);
    println!("Square of 4 is: {}", result);
}

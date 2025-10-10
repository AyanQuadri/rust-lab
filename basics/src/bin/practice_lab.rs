// fn main() {
//     println!("Hello, rust!");
// }

// fn main(){
//     // Taking the first number in x
//     let x = 5;
//     // Taking the second number in y
//     let y = 3;
//     // Adding both x and y and storing it in sum
//     let sum = x + y;
//     // Printing the sum
//     println!("The sum of {x} {y} is {}", sum);
// }

// fn main() {
//     let mut x = 10;
//     println!("Initial value: {x}");
//     x = 20;
//     println!("Final value: {x}");
// }

// fn main(){
//     let int_value: i32 = 42;
//     let float_value: f64 = 3.14;
//     println!("Integer value: {}", int_value);
//     println!("Floating-point value: {}", float_value);
// }

// fn main(){
//     let is_rust_fun: bool = true;
//     let chara:char = 'R';
//     println!("Is Rust fun? {}", is_rust_fun);
//     println!("The letter is: {}", chara);
// }

// fn main(){
//     let person:(i32, f32, char) = (30, 5.8, 'M');
//     println!("Age: {}", person.0);
//     println!("Height: {}", person.1);
//     println!("Gender: {}", person.2);
// }

// fn main() {
//     let mut numbers = [1, 2, 3, 4, 5];
//     numbers[2] = 10;
//     println!("The array is: {:?}", numbers);
// }

// fn main() {
//     const MAX_POINTS: u32 = 100_000;
//     let mut points = 0;
//     println!("Maximum points: {}", MAX_POINTS);
//     println!("Current points: {}", points);
//     points = 10_000;
//     println!("Updated points: {}", points);
// }

// fn main(){
//     let x = 5;
//     println!("The value of x is: {}", x);
//     let x = x + 1;
//     println!("The value of x is: {}", x);
//     let x = x * 2;
//     println!("The value of x is: {}", x);
// }

// macro_rules! print_message {
//     () => {
//        println!("This is a macro message!");
//     };
// }
// fn main(){
//     print_message!();
// }

macro_rules! print_custom_message {
    ($msg:expr) => {
        println!("{}", $msg);
    };
}
fn main() {
    print_custom_message!("Hello from macro!");
    print_custom_message!("This is another message.");
}

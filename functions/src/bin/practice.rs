// fn main() {
//     print_message();
// }
// fn print_message() {
//     println!("Rust is great!")
// }

// fn main() {
//     greet_user("Alice");
// }
// fn greet_user(name: &str) {
//     println!("Hello, {}!", name)
// }

// fn main() {
//     let product = calculate_product(5, 7);
//     println!("The product is: {}", product);
// }
// fn calculate_product(num1: i32, num2: i32) -> i32 {
//     num1 * num2
// }

// fn main() {
//     let message = welcome_message("Alice");
//     println!("{}", message);
// }
// fn welcome_message(name: &str) -> String {
//     format!("Welcome, {}!", name)
// }

// fn main() {
//     let number = 10;
//     let result = is_even(number);
//     println!("Is {} even? {}", number, result);
// }
// fn is_even(num: i32) -> bool {
//     num % 2 == 0
// }

// fn main() {
//     let difference = calculate_difference(10, 3);
//     println!("The difference is: {}", difference);
// }
// fn calculate_difference(a: i32, b: i32) -> i32 {
//      a - b
// }

// fn main() {
//     let descriptions = [describe_number(-1), describe_number(0), describe_number(2)];
//     for descriptions in &descriptions {
//         println!("{}", descriptions);
//     }
// }
// fn describe_number(num: i32) -> &'static str {
//     match num {
//         num if num < 0 => "Negative",
//         0 => "Zero",
//         _ => "Positive",
//     }
// }

// fn main() {
//     let result = factorial(5);
//     println!("The factorial of 5 is: {}", result);
// }
// fn factorial(n: i32) -> i32 {
//     if n <= 1 { 1 } else { n * factorial(n - 1) }
// }

// fn main() {
//     greet_optionally(Some("Alice"));
//     greet_optionally(None);
// }
// To take null values
// fn greet_optionally(name: Option<&str>) {
//     match name {
//         Some(n) => println!("Hello, {}!", n),
//         None => println!("Hello, Stranger!"),
//     }
// }

// fn main() {
//     match divide(10.0, 2.0) {
//         Ok(result) => println!("Result: {}", result),
//         Err(e) => println!("Error: {}", e),
//     }
//     match divide(10.0, 0.0) {
//         Ok(result) => println!("Result: {}", result),
//         Err(e) => println!("Error: {}", e),
//     }
// }
// fn divide(a: f64, b: f64) -> Result<f64, &'static str> {
//     if b == 0.0 {
//         Err("Cannot divide by zero")
//     } else {
//         Ok(a / b)
//     }
// }

fn main() {
    let result1 = apply_twice(double, 2);
    println!("Double twice of 2: {}", result1);
    let result2 = apply_twice(increment, 2);
    println!("Increment twice of 2: {}", result2);
}
fn apply_twice(f: fn(i32) -> i32, x: i32) -> i32 {
    f(f(x))
}
fn double(x: i32) -> i32 {
    x * 2
}
fn increment(x: i32) -> i32 {
    x + 2
}

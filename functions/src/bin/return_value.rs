use std::io::{self, Write};

fn main() {
    // send parameters and store the return value in add var
    let add = add(5, 5);
    println!("The sum is {}", add);

    // Get the name from the user input
    print!("Write the name to say hello to: ");
    io::stdout().flush().expect("couldn't flush");
    let mut name = String::new();

    // Read line
    io::stdin().read_line(&mut name).expect("cannot input");
    let name = name.trim();
    let message = greet(name);
    println!("{}", message);
}
// we should explictly mention the return value datatype
fn add(num1: i32, num2: i32) -> i32 {
    // for some reason return isn't working so write it directly
    num1 + num2
}
fn greet(name: &str) -> String {
    // don't use semiclons for returning value
    // ; turns expression in statement
    format!("Hello, {}!", name)
}

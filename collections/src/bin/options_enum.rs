use std::io::{self, Write};

fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    // if denominator is zero
    if denominator == 0.0 {
        None
    } else {
        // if the denominator is not zero
        Some(numerator / denominator)
    }
}

fn main() {
    // Ask for the numerator
    print!("Write the value for the numerator: ");
    // flush out so that i can be print before asking the value
    io::stdout().flush().expect("Couldn't flush out");
    let mut input = String::new();
    // Read the input in the input variable
    io::stdin()
        .read_line(&mut input)
        .expect("Couldn't read the line");
    // parse the value to be in float
    let a: f64 = input.trim().parse().expect("Write a float value: ");

    // Ask for the denominator
    print!("Write the value for the denominator: ");
    // flush out so that i can be print before asking the value
    io::stdout().flush().expect("Couldn't flush out");
    let mut input = String::new();
    // Read the input in the input variable
    io::stdin()
        .read_line(&mut input)
        .expect("Couldn't read the line");
    // parse the value to be in float
    let b: f64 = input.trim().parse().expect("Write a float value: ");

    let result = divide(a, b);

    // they are two values in options some and none, so that we can recieve null values
    match result {
        None => println!("Cannot divide the numerator with zero!"),
        Some(value) => println!("The Answer is: {}", value),
    }
}

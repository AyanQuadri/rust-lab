use std::io::{self, Write};

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Couldn't divide numerator with 0"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    let mut a = String::new();
    print!("Write the value of the numerator: ");
    io::stdout().flush().expect("couldn't flush the buffer");
    io::stdin()
        .read_line(&mut a)
        .expect("coudln't read the value");
    let numerator: i32 = a.trim().parse().expect("couldn't parse the value");

    let mut b = String::new();
    print!("Write the value of the denominator: ");
    io::stdout().flush().expect("couldn't flush the buffer");
    io::stdin()
        .read_line(&mut b)
        .expect("coudln't read the value");
    let denominator: i32 = b.trim().parse().expect("couldn't parse the value");

    match divide(numerator, denominator) {
        Ok(value) => println!("After dividing we get: {}", value),
        Err(e) => println!("Error: {}", e),
    }
}

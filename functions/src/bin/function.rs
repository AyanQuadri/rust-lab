use std::io::{self, Write};

fn main() {
    // Writing a simple function without parameter
    greet();
}
fn greet() {
    print!("Write the name you wanna greet!: ");
    io::stdout().flush().expect("couldn't flush");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("couldn't input the line");
    let name = name.trim();
    println!("Hello, {}!", name);
}

use std::io::{self, Write};

fn main() {
    let a = 10;
    println!("Original a value is {}", a);
    let a = 5 + 1;
    println!("The value of a is {}", a);
    {
        let a = 4 + 15;
        println!("The value of a in the scope is {}", a);
    }
    let a = 10 + 1;
    println!("The value of a outside the scope is {}", a);

    // Different between mutability and shadowing
    // Mutability
    let mut count = 5;
    println!("The count is {}", count);
    count = 5 + 1;
    println!("The count is {}", count);

    // Shadowing we can even change the datatype of the variable
    let count = "Hello";
    println!("The count is {}", count);
    let count = count.len();
    println!("The count is {}", count);

    // Taking input and performing trim on a string
    let mut word = String::new();
    print!("Write the word: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut word)
        .expect("Failed to get the text");
    let word = word.trim();
    println!("The thing you wrote it: {}", word);
}

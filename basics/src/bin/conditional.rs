use std::io::{self, Write};

fn main() {
    let mut num = String::new();
    print!("Write the number: ");
    // Flushing so that it prints the value firt 
    io::stdout().flush().expect("Couldn't flush");
    io::stdin()
        .read_line(&mut num)
        .expect("Can't read the number");
    // Changing the string to a number using parse
    let num: i32 = num.trim().parse().expect("It's not a integer");

    if num % 5 == 0 && num % 2 == 0 {
        println!("It satisfies the first condition could be divided by '5 and 2'");
    } else if num % 5 == 0 {
        println!("It satisfies the second condition could only be divided by '5'");
    } else {
        println!("It doesn't satisfy any condition");
    }
}

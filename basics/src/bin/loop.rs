use std::io::{self, Write};

fn main() {
    let mut num = String::new();
    print!("Write the number you want the table of: ");
    io::stdout().flush().expect("Failed to flush");
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read the number");
    let num: u32 = num
        .trim()
        .parse()
        .expect("Couldn't change the string to number");

    // Fisrt declare the variable
    let mut count: u32 = 0;
    loop {
        count += 1;
        if count <= 10 {
            let multiplication: u32 = num * count;
            println!("{} * {} = {}", num, count, multiplication);
        } else {
            // You can also return value using break return_value;
            break;
        }
    }
}

use std::io::{self, Write};

fn main() {
    // normal match control flow
    let mut number = String::new();
    print!("Write the value of the number: ");
    io::stdout().flush().expect("couldn't flush!");
    io::stdin()
        .read_line(&mut number)
        .expect("couldn't read the line!");
    let number = number.trim().parse().expect("couldn't parser the string!");
    match number {
        0 => println!("zero"),
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Invalid"),
    }

    // operators
    let mut number_second = String::new();
    print!("Write the value of the second number: ");
    io::stdout().flush().expect("Coudln't flush");
    io::stdin()
        .read_line(&mut number_second)
        .expect("couldn't read the line!");
    let number_second = number_second
        .trim()
        .parse()
        .expect("couldn't parser the string!");
    match number_second {
        1 | 2 | 3 => println!("It's either 1 or 2 or 3"),
        _ => println!("Invalid"),
    }

    // Range
    let mut number_third = String::new();
    print!("Write the third number: ");
    io::stdout().flush().expect("couldn't flush");
    io::stdin()
        .read_line(&mut number_third)
        .expect("Coudn't read the line!");
    let number_third = number_third.trim().parse().expect("couldn't be parsed!");
    match number_third {
        1..=5 => println!("The number lies between 1 and 5"),
        6..=10 => println!("The number lies between 5 and 10"),
        _ => println!("Something else"),
    }

    // destructuring the tuple, it stores the tupple.0, tupple.1 in x, y

    let mut pair: Vec<i32> = Vec::new();
    let elements: u8 = 2;
    let mut input = String::new();
    for i in 0..elements {
        input.clear(); // <-- this clears the string for next input
        print!("Write the value of the {} vector: ", i + 1);
        io::stdout().flush().expect("couldn't flush");
        io::stdin().read_line(&mut input).expect("coudln't input");
        let input: i32 = input.trim().parse().expect("couldn't be parsed");
        pair.push(input);
    }

    match &pair[..] {
        [x, y] if x == y => println!("The values are same!"),
        [x, y] if x + y == 0 => println!("The values are opposite!"),
        [x, y] => println!("no special properties for {}, {}", x, y),
        _ => println!("Need two numbers!"),
    }
}

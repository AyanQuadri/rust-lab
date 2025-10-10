use std::io::{self, Write};
fn main() {
    let numbers = [1, 2, 3, 4, 5];

    for num in numbers {
        println!("The values of array is {}", num);
    }
    for i in 1..5 {
        print!("{}, ", i);
        io::stdout().flush().expect("Couldn't flush");
    }
    println!();

    let mut while_number = 3;
    while while_number != 0 {
        println!("number: {}", while_number);
        while_number -= 1;
    }

    let mut vector_arr: Vec<i32> = Vec::new();
    let mut elements = String::new();
    print!("Write the number of elements you want in the array: ");
    io::stdout().flush().expect("Couldn't flush");
    io::stdin()
        .read_line(&mut elements)
        .expect("couldn't read the value");
    let elements: u8 = elements.trim().parse().expect("cound't be parsed");

    for i in 0..elements {
        print!("Enter the {} value: ", i + 1);
        io::stdout().flush().expect("can't flush");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("couldn't input the value");

        let value: i32 = input.trim().parse().expect("Coudn't be parsed");
        vector_arr.push(value);
    }
    println!("The array is {:?}", vector_arr);
}

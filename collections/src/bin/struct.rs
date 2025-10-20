// name should be capital
struct Rectangle {
    width: u32,
    length: u32,
}
fn main() {
    // initializing the struct in a variable
    let r1 = Rectangle {
        width: 30,
        length: 50,
    };

    // printing each value from r1
    println!("width: {}", r1.width);
    println!("length: {}", r1.length);

    // declare values to use in r2
    let width = 40;
    let length = 60;

    let r2 = Rectangle {
        // You can only write this because of rust's shorthand form!
        // could only use when we have the same name variables in the main and the struct
        width,
        length,
    };

    // this will transfer the ownership to r4 and r2 will become invalid
    let r4 = r2;

    // printing each value from r2
    println!("width: {}", r4.width);
    println!("length: {}", r4.length);

    // we can also inherit remaining values from other variables
    let r3 = Rectangle { length: 70, ..r1 };

    //printing r3 values
    println!("width: {}", r3.width);
    println!("length: {}", r3.length);
}

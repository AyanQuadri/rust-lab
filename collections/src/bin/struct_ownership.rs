struct Rectangle {
    width: u32,
    length: u32,
}

// borrowing the struct
// if we don't use '&' it will change the ownership
fn print_dimensions(r2: &Rectangle) {
    println!("width: {}", r2.width);
    println!("length: {}", r2.length);
}

// modify it by using mut
fn modify_dimensions(r2: &mut Rectangle) {
    r2.width = 50;
    r2.length = 80;
}

fn main() {
    let r1 = Rectangle {
        width: 30,
        length: 50,
    };

    // Giving ownership to r2 from r1
    let mut r2 = r1;

    // Giving a reference to r2
    print_dimensions(&r2);

    //printing r2 as r1 isn't valid anymore
    println!("width: {}", r2.width);
    println!("length: {}", r2.length);

    modify_dimensions(&mut r2);

    // print the modified Rectangle
    println!("width: {}", r2.width);
    println!("length: {}", r2.length);
}

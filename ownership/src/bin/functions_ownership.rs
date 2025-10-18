fn main() {
    let s = String::from("Hello");
    let x = 5;
    take_ownership(s);
    copy_trait_integer(x);
    println!(
        "This will print as integers are copied becuase they live in stack: {}",
        x
    );
    /*
     * println!("s:{}", s);
     * you will not have access to the value
     * as the owner for the value in the heap is changed
     * */

    // This will get the string ownership from the function
    let s1 = get_ownership();
    let s2 = String::from("World");
    // This functiion will take and give the ownership to s3
    let s3 = transfer_ownsership(s2);

    println!("{}, {}!", s1, s3);

    let modify_string = String::from("Goodbye");
    let reuslt = updating_string(modify_string);

    println!("Updated string: {}", reuslt);

    let x1 = gives_copy();
    let x2 = 8;
    let x3 = takes_and_return_copy(x2);

    println!("x1: {}, x2: {}, x3: {}", x1, x2, x3);
}

// passing ownership
fn take_ownership(s: String) {
    println!("s: {}", s);
}

fn copy_trait_integer(x: i32) {
    println!("x: {}", x);
}

fn get_ownership() -> String {
    String::from("Hello")
}

fn transfer_ownsership(s2: String) -> String {
    s2
}

fn updating_string(mut modify_string: String) -> String {
    modify_string.push_str(", world!");
    modify_string
}

fn gives_copy() -> i32 {
    5
}

fn takes_and_return_copy(mut _x2: i32) -> i32 {
    _x2 = 10;
    _x2
}

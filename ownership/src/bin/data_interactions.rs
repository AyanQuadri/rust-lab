fn main() {
    // for integers
    let x = 10;
    let y = x;
    println!("x: {}, y: {}", x, y);
    /* The integers are stored on stack
    and implement the copy trait */

    // for Strings
    let s1 = String::from("Hello");
    let s2 = s1;
    println!("s2: {}", s2);
    // we can't use s1 as the ownership has been changed
}

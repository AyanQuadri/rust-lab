fn main() {
    let x = 5;
    let y = x;
    // Uses the copy trait, bits are copied in stack
    println!("x: {}, y: {}", x, y);

    let s1 = String::from("Hello");
    let s2 = s1.clone();
    // Uses the clone trait,
    // the memory stored in heap is copied exactly
    println!("s1: {}, s2: {}", s1, s2);
}

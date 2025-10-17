fn main() {
    let mut s = "hello";
    println!("String literal which is in the stack: {}", s);

    s = "world";
    println!("String literal after updating: {}", s);

    let mut sobj = String::from("Hello");
    println!("String which is stored in the heap: {}", sobj);

    sobj.push_str(" world");
    println!("After pushing: {}", sobj);
}

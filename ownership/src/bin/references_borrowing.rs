fn main() {
    let s = String::from("Hello");
    let length = calculating_len(&s);

    // This will print s, because it's still valid
    println!("The length of string: {} is {}", s, length);
}

// Passing a reference of the variable to the function
fn calculating_len(_s: &String) -> usize {
    _s.len()
}

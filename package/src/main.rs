fn main() {
    let reversed: String = text_library::reverse("Rust");
    println!("Reversed value: {}", reversed);

    let result = text_library::maths::add(5, 19);
    println!("Result = {}", result);

    let result = text_library::maths::sub(8, 5);
    println!("Result = {}", result);
}

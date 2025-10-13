fn main() {
    greet("Bob");
    greet("Alice");
    let area = calc_area(10, 20);
    println!("The area if {}", area);
    let message1 = format_string("Error", "File not found");
    let message2 = format_string("Warning", "Low disk space");
    println!("{}", message1);
    println!("{}", message2);
}
fn greet(name: &str) {
    println!("Hello, {}!", name);
}
fn calc_area(width: i32, height: i32) -> i32 {
    // We don't write ; after it because it would not return value
    // it will think it as the end of the function
    width * height
}
fn format_string(prefix: &str, message: &str) -> String {
    format!("[{}], {}", prefix, message)
}

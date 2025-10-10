macro_rules! greet1 {
    () => {
        println!("Hello, world!");
    };
}
macro_rules! greet2 {
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}
fn main() {
    greet1!();
    greet2!("Kunal");
}

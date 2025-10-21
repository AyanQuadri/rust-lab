use core::fmt;

// or if we don't want to write our custom formatting
// we can simple write before the struct
// #[allow(dead_code)] for deadcode
// #[derive(Debug)] for debugging and printing with {:?}
struct Rectangle {
    x: u32,
    y: u32,
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let r1 = Rectangle { x: 50, y: 80 };

    println!("Rectangle dimensions: {}", r1);
    println!("x:{}, y:{}", r1.x, r1.y); // this is unnecessary but you should write this to avoid
    // warnings of unsued things
}

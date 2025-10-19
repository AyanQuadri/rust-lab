use std::{self, time::Instant};
fn main() {
    let long_string = "a".repeat(100000);
    let start = Instant::now();
    let _clone_string = long_string.clone();
    let duration = start.elapsed();
    println!("Time taken to clone: {:?}", duration);

    let start = Instant::now();
    let _moved_string = long_string;
    let duration = start.elapsed();
    println!("Time taken to move: {:?}", duration);
}

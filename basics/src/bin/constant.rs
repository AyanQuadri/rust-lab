const IP_ADDRESS: &str = "192.168.0.10";
fn main() {
    // Constants are always immutable, in simple words it cannot change
    const MAX_VALUE: u32 = 500;
    println!("The max value is {}", MAX_VALUE);
    println!("The ip address is {}", IP_ADDRESS);
}

fn main() {
    let x = 10;
    println!("The value of x is {}", x);
    /*
     * you can't mutate a value without explicatly mentioning that it can be mutable
     * x = 20;
     */
    println!("The value of x is {}", x);
    let mut y = 20;
    println!("The value of y is {}", y);
    y = 40;
    println!("The value of y is {}", y);
    y = 50;
    println!("The value fo y is {}", y);
}

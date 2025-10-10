fn main() {
    //Tuples in rust
    let tup: (i32, f64, u8) = (-599, 4.252, 199);
    println!("{} {} {}", tup.0, tup.1, tup.2);
    let first_tuple = tup.0;
    println!("{}", first_tuple);

    let (x, y, z) = tup;
    println!(
        "The value of x is {}, value of y is {}, value of z is {}",
        x, y, z
    );

    //Arrays
    let a = [1, 2, 3, 4, 5];
    /* Writing an index that doesn't exist in the array gives an error saying program panicked
     * this happens because of rust's memory safety
     * let index = 6;
     */
    let index_practice = 4;
    let index = 6;
    let element_practice = a[index_practice];
    println!(
        "The value of the element at index {} is {}",
        index_practice, element_practice
    );

    match a.get(index) {
        Some(element) => println!("The value of the element at index {index} is {element}"),
        None => println!("Invalid Index!"),
    }
}

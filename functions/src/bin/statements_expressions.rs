// main function is also a statement
fn main() {
    /*
     * This creates a statement
     * statement doesn't return value
     */
    let y = 6;
    println!("{}", y);

    /* These arithmetic is an expression which will return some value
     * and also function calls which returns value
     * and also block of code is also an example of expressions
     */
    let z = z_value(5, 6);
    println!("{}", z);

    let x = {
        let a = 5;
        a + 2
    };
    println!("{}", x);

    // If as an expression:-
    let condition: bool = true;
    let b = if condition { 15 } else { 10 };
    println!("{}", b);

    // Match as a expression
    let match_example = 2;
    let result = match match_example {
        1 => "One",
        2 => "Two",
        3 => "Three",
        _ => "invalid",
    };
    println!("{}", result);
}
fn z_value(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

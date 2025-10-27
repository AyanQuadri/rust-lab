fn main() {
    // Define an Option that currently has no value
    let opt: Option<i32> = None;
    let result: Result<i32, &str> = Err("An error occured!");

    // Try to get the value, or else run the closure to get a default
    let value = opt.unwrap_or_else(|| {
        // This code runs only if opt is None
        println!("No value found, returning default value!");
        10 // return 10 as the default value
    });

    println!("The value is: {}", value);

    let value = result.unwrap_or_else(|err| {
        println!("Error encountered: {}", err);
        -1 // Fall back value
    });
    println!("The result: {}", value);
}

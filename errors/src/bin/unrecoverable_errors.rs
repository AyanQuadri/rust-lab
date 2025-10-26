fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("can't divide it by 0"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    let v = vec![1, 2, 3];

    println!("The element in:{:?}", v);

    println!("The second element is:{}", v[2]);

    // this will panic the program and exit the program
    // goind backwards in the stack and reallocating memory
    // then provide the info
    // and unrecoverable errros will stop the program completely
    // the after part will not run

    // We can use the panic macro
    /*    if v.len() <= 3 {
            panic!("There is no fourth value");
        } else {
            println!("The fourth element is:{}", v[3]);
        }
    */
    // We can use ok and err to safely handle erros in a program
    match divide(10, 0) {
        Ok(value) => println!("The value after divison is: {}", value),
        Err(e) => println!("Error: {}", e),
    }
}

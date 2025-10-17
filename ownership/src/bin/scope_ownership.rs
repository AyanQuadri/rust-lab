fn main() {
    {
        let x = 10; // We define 'x' here
                    // it will remain in the scope
        println!("The value of x is: {}", x);
    } // Afte this block x value will be dropped as it gets out of scope
      /*    println!("The value of x is: {}", x);
        this will give a error of the variable is out of scope
      which says the value is dropped
           */
}

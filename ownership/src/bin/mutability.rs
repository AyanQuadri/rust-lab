fn main() {
    let mut s = String::from("Hello");

    // can make a mutable reference which can change the value
    let r = &mut s;
    r.push_str(", World!");

    let h = &mut s;
    // after this line r will become invalid because
    // rust allows only one borrow for mutable reference to exist
    h.push_str("I'm still here!");

    /* when we print s here h becomes invalid
    we can't print h after printing s
    as it isn't valid anymore
    more precisely as we used the original value
    the borrow becomes invalid */
    println!("s: {}", s);

    let _r;
    {
        let x = 5;
        _r = &x;
        println!(
            "It is valid only till here, it stops the dangling reference becuase 'x' has gotten out of socpe, The value is: {}",
            _r
        );
    }
    /*
    println!("r: {}", r);
    This will give an compile time error
    */
}

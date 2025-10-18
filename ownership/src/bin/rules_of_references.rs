fn main() {
    let mut s = String::from("Hello");
    {
        // can make multiple immutables
        let r1 = &s; // immutable - 1
        let r2 = &s; // immutable - 2
        println!("r1: {}, r2: {}", r1, r2); // They both are valids
    } // immutable references goes out of scope

    let r1 = &s; // immutable - 1
    let r2 = &s; // immutable - 2
    println!("r1: {}, r2: {}", r1, r2);
    // this will still work because r1 and r2 has been used

    /* can only have one mutable reference to ensure memory safety
     * and avoid data races
     * you can't also have immutable and mutable to be mix up
     * one mutable or multiple immutables */

    let m = &mut s;
    m.push_str(", World!");

    /*
     * println!("r1: {}, r2: {}", r1, r2);
     * This won't work as it doesn't allow use to use immutable while having mutable
     * */

    println!("m: {}", m); // mutable reference still valid
    println!("s: {}", s); // now the mutable reference m will not be valid after this

    /*
     * println!("m: {}", m);
     * This won't work
     * */
}

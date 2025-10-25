fn main() {
    // Vectors are dynamic arrays that can grow and shrink
    let mut v: Vec<i32> = Vec::new();

    // vec! is a macro used to create vector with initial values
    let mut v2 = vec![1, 2, 3];

    // They are stored on the heap memory as Strings
    // push method add the value at the last
    v.push(5);
    v.push(10);
    v.push(15);
    v.push(20);

    println!("vector: {:?}", v);

    println!("v2: {:?}", v2);

    // This will pop the last value
    v2.pop();

    // Insert method is used to insert a element in a specific index
    v2.insert(1, 5);

    println!("v2: {:?}", v2);

    // Will let you access the specific index
    let third: &i32 = &v[1];
    println!("third: {}", third);

    // we can also use match to print a certain index which can protect you from
    // panicing your program
    match v.get(2) {
        Some(value) => println!("Third element is: {}", value),
        None => println!("There is not third element"),
    }
}

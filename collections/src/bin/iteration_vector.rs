fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    for i in &v {
        print!("{} ", i);
    }

    // this works the same as giving the reference to the vector
    for i in v.iter() {
        print!("{} ", i);
    }

    // To mutate the vector
    for i in &mut v {
        // Here you are dereferencing the array
        // or you would be adding integer to meta data of the vector
        // because i is just pointing towards data in the heap it doesn't own the data
        // it just have the meta data that is pointer, lenght and memory
        *i += 1;
    }
    println!("{:?}", v);

    // printing the length of the vector
    println!("The length of the vector: {}", v.len());

    // To check if the vector contains certain values
    // it will answer in boolean
    println!("Does the vector contains 2: {}", v.contains(&2));

    let v1: Vec<i32> = Vec::new();
    // to check if the vector is empty or not
    println!("Is the vector empty: {}", v1.is_empty());
}

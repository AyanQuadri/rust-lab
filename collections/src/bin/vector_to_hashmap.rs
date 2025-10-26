use std::collections::HashMap;

fn main() {
    // Make two vectors
    let names = vec!["Alice", "Bob", "Charlie"];
    let grades = vec![80, 88, 94];

    // .into_iter() is used to take ownership from the vector where as iter() expects & meaning
    // a reference  into_iter takes T as value not &T
    // zip combines two iter in a single pair tuples
    // collect collects the pair as defined in the start for the below we have hashmap at the start
    let student: HashMap<_, _> = names.into_iter().zip(grades.into_iter()).collect();

    println!("student: {:?}", student);
}

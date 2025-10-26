use std::collections::HashMap;

fn main() {
    let mut students = HashMap::new();

    students.insert("Alice", 92);
    students.insert("Bob", 82);
    students.insert("Charlie", 99);

    for (student, grades) in &students {
        println!("{}: {}", student, grades);
    }

    // To print number of students in the hashmap
    println!("Number of students: {}", students.len());

    // To check if the hashmap is empty or not
    println!("is the map empty? {}", students.is_empty());

    // To check if certain key exist
    println!("Does Alice exists:{}", students.contains_key("Alice"));
}

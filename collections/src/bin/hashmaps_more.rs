use std::collections::HashMap;

fn main() {
    let mut student_grade = HashMap::new();

    student_grade.insert("Alice", 92);

    // Access the value with get
    let grade = student_grade.get("Alice");

    match grade {
        Some(g) => println!("Alice marks are: {}", g),
        None => println!("Couldn't find Alice"),
    }

    // inserting with the same key will update the value
    student_grade.insert("Alice", 82);
    // This will check if alice exist or not
    // if it exist nothing will be changed if not it will write the value
    student_grade.entry("Alice").or_insert(88);

    // we can also use this direct method to print the value from the key
    println!("Alice marks are: {}", student_grade["Alice"]);

    // We can remove the key and it's value by using .remove
    student_grade.remove("Alice");
}

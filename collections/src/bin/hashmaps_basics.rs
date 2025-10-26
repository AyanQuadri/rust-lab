use std::collections::HashMap;

fn main() {
    // to created a empty HashMap
    let mut student_grades = HashMap::new();

    // insert values as names being the key and numbers being the value
    // the metadata is stored on the stack and the value is stored in the heap
    // if you have the same key in it the previous one will become invalid
    // if i add another alice that will become the original value
    student_grades.insert("Alice", 1);
    student_grades.insert("Bob", 2);
    student_grades.insert("Kunal", 3);

    // printing all the values in the hashmaps
    println!("Student grades:{:?}", student_grades);
}

use std::collections::HashMap;

fn main() {
    let mut student_grades: HashMap<String, i32> = HashMap::new();
    student_grades.insert(String::from("Alice"), 92);

    let mut new_student_grades = student_grades;

    // We can't print student_grades as the new_student_grades has the ownership
    println!("After changing the ownership: {:?}", new_student_grades);

    // borrows the value we can still use both of them simultaneously
    let borrowed_grades = &new_student_grades;

    println!("After borrwing the value: {:?}", new_student_grades);
    println!("From the borrowing variable: {:?}", borrowed_grades);

    // make a new variable that can insert new values
    let mut_borrowed_grades = &mut new_student_grades;
    mut_borrowed_grades.insert(String::from("Bob"), 80);

    /*
    println!("New student grades: {:?}", borrowed_grades);
    this will give an error because only one is allowed either mutable or immutable
    */

    println!(
        "After updating with another variable that have reference to the mutable original value: {:?}",
        new_student_grades
    );

    /*println!("New student grades: {:?}", mut_borrowed_grades);
     * this will also give an error as we print the main value this variable goes out of scope
     * */
}

fn print_vector(v: &Vec<i32>) {
    println!("vector inside the function: {:?}", v);
}

fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = v1;

    // The ownership has been transferred from v1 to v2
    // we can't print v1 now
    println!("v2: {:?}", v2);

    let first = &v2[0];
    println!("first value without taking the ownership: {}", first);

    print_vector(&v2);

    println!(
        "v2 after borrowing the first value to first and also borrowing for the function: {:?}",
        v2
    );
}

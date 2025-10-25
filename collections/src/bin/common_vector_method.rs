fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    // To remove value from a certain index
    let removed = v.remove(1);

    println!("Removed value: {}", removed);
    println!("Updated vector: {:?}", v);

    // This will add the values to the attend
    v.extend(vec![6, 7, 8, 9]);
    println!("Updated vector: {:?}", v);
}

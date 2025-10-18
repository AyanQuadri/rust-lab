fn main() {
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..4];
    println!("slice: {:?}", slice);

    let s = String::from("Hello, World!");
    let hello = &s[0..5];
    let world = &s[7..12];

    println!("{}, {}!", hello, world);

    let mut arr2 = [1, 2, 3, 4, 5];
    let slice2 = &mut arr2[0..5];
    slice2[0] = 10;
    println!("arr2: {:?}", arr2);
}

// fn main() {
//     {
//         let x = 10;
//         println!("x is valid inside the block: {}", x);
//     }
// }

// fn main() {
//     let s = create_string();
//     println!("String in main: {}", s);
// }
// fn create_string() -> String {
//     let s = String::from("hello");
//     println!("String inside function: {}", s);
//     s
// }

// fn main(){
//     let x = 10;
//     let y = x;
//     println!("x: {}, y: {}", x, y);
//     let s1 = String::from("hello");
//     let s2 = s1;
//     println!("s2: {}", s2);
// }

// fn main() {
//     let s = String::from("hello");
//     take_ownership(s);
// }
// fn take_ownership(s: String) {
//     println!("String inside function: {}", s);
// }

// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1;
//     println!("s2: {}", s2);
// }

// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1.clone();
//     println!("s1: {}, s2: {}", s1, s2);
//     let s3 = String::from("world");
//     let s4 = s3;
//     println!("s4: {}", s4);
// }

// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1.clone();
//     takes_ownership(s2);
//     println!("s1 is still valid: {}", s1);
// }
// fn takes_ownership(s: String) {
//     println!("String inside function: {}", s);
// }

// fn main(){
//     let s1 = String::from("hello");
//     let s2 = clone_and_return(s1.clone());
//     println!("s1: {}, s2: {}", s1, s2);
// }
// fn clone_and_return (s: String) -> String {
//     s.clone()
// }

// use std::time::Instant;
// fn main() {
//     let large_string = "a".repeat(100000);
//     let start = Instant::now();
//     let _cloned_string = large_string.clone();
//     let duration = start.elapsed();
//     println!("Time taken to clone: {:?}", duration);
//     let start = Instant::now();
//     let _moved_string = large_string;
//     let duration = start.elapsed();
//     println!("Time taken to move: {:?}", duration);
// }

// fn main() {
//     let mut s1 = String::from("hello");
//     let _len1 = calculate_length_ownership(s1);
//     // println!("The length of {} is {}", s1, len1); // s1 is no longer valid
//     let s2 = String::from("world");
//     let len2 = calculate_length_borrowing(&s2);
//     println!("The length of {} is {}", s2, len2);
// }
// fn calculate_length_ownership(s: String) -> usize {
//     s.len()
// }
// fn calculate_length_borrowing(_s: &String) -> usize {
//     _s.len()
// }

// fn main() {
//     let x = 10;
//     let r1 = &x;
//     let r2 = &x;
//     println!("r1: {}, r2: {}", r1, r2);
// }

// fn main() {
//     let mut s = String::from("hello");
//     let r = &mut s;
//     r.push_str(", world");
//     println!("s: {}", s);
// }

// fn main() {
//     let _r;
//     {
//         let x = 5;
//         _r = &x;
//     }
// }

// fn main() {
//     let mut s = String::from("hello");
//     {
//         let r1 = &s;
//         let r2 = &s;
//         println!("r1: {}, r2: {}", r1, r2);
//     }
//     let r3 = &mut s;
//     r3.push_str(", world");
//     println!("r3: {}", r3);
// }

// fn main(){
//     let arr = [1, 2, 3, 4, 5];
//     let slice = &arr[1..4];
//     println!("{:?}", slice);
// }

// fn main() {
//     let s = String::from("hello, world");
//     let hello = &s[0..5];
//     let world = &s[7..12];
//     println!("{}, {}", hello, world);
// }

// fn main() {
//     let mut arr = [1, 2, 3, 4, 5];
//     let slice = &mut arr[1..4];
//     slice[0] = 10;
//     println!("{:?}", arr);
// }

// fn main() {
//     let _slice;
//     {
//         let arr = [1, 2, 3, 4, 5];
//         _slice = &arr[1..4];
//     }
// }

fn main() {
    let mut s = String::from("hello");
    {
        let slice1 = &s[0..5];
        let slice2 = &s[0..5];
        println!("slice1: {}, slice2: {}", slice1, slice2);
    }
    let _slice3 = &mut s[0..5];
    s.replace_range(0..5, "hi");
    println!("s: {}", s);
}


// fn main() {
//     let number: i32 = 10;
//     if number > 0 {
//         println!("The number is positive.");
//     } else if number == 0 {
//         println!("The number is zero.");
//     } else {
//         println!("The number is negative.")
//     }
// }

// fn main() {
//     let number = 8;
//     if number >= 1 && number <= 10 {
//         println!("The number is within the range 1 to 10.");
//     } else {
//         println!("The number is outside the range 1 to 10.");
//     }
// }

// fn main() {
//     let mut number = 1;
//     loop {
//         if number > 5 {
//             break;
//         }
//         println!("Number: {}", number);
//         number += 1;
//     }
// }

// fn main() {
//     let mut number = 10;
//     let result = loop {
//         if number > 10 && number % 3 == 0 {
//             break number;
//         }
//         number += 1;
//     };
//     println!(
//         "The first number greater than 10 that is divisible by 3 is: {}",
//         result
//     );
// }

// fn main() {
//     let mut outer_count = 0;
//     'outer: loop {
//         println!("Entering outer loop");
//         let mut inner_count = 0;
//         loop {
//             println!("Inner count: {}", inner_count);
//             inner_count += 1;
//             if inner_count == 2 {
//                 break;
//             }
//             println!("Outer count: {}", outer_count);
//             if outer_count == 2 {
//                 break 'outer;
//             }
//         }
//         outer_count += 1;
//     }
//     println!("Exited the outer loop");
// }

// fn main() {
//     for i in 1..6 {
//         if i == 3 {
//             continue;
//         }
//         println!("{}", i);
//     }
// }

// Using loop, not for for the above program
// fn main() {
//     let mut number = 0;
//     loop {
//         number += 1;
//         if number == 3 {
//             continue;
//         } else if number == 6 {
//             break;
//         }
//         println!("Number: {}", number);
//     }
// }

// fn main() {
//     let numbers: [i32; 5] = [1, 2, 3, 4, 5];
//     for i in numbers.iter() {
//         println!("Number: {}", i);
//     }
// }

// fn main() {
//     for number in 1..=10 {
//         println!("Number: {}", number);
//     }
// }

// fn main() {
// let mut number = 1;
// while number <= 5 {
// println!("Number: {}", number);
// number += 1;
// }
// }

// fn main() {
//     let mut number = 1;
//     while number <= 10 {
//         if number % 2 == 0 {
//             println!("Even number: {}", number);
//         }
//         number += 1;
//     }
// }

// fn main() {
//     for number in 1..=10 {
//         if number == 3 {
//             continue;
//         }
//         if number == 5 {
//             break;
//         }
//         println!("Number: {}", number);
//     }
// }

// fn main() {
//     let mut number = 0;
//     while number < 10 {
//         number += 1;
//         if number == 4 {
//             continue;
//         }
//         if number == 7 {
//             break;
//         }
//         println!("Number: {}", number);
//     }
// }

// fn main() {
//     let number = 3;
//     match number {
//         1 => println!("One"),
//         2 => println!("Two"),
//         3 => println!("Three"),
//         _ => println!("Other"),
//     }
// }

// fn main(){
//     let number = 2;
//     match number {
//         1 | 2 | 3 => println!("One, Two, or Three"),
//         _ => println!("Other"),
//     }
// }

// fn main() {
//     let number = 5;
//     match number {
//         1..=10 => println!("Between 1 and 10"),
//         11..=20 => println!("Between 11 and 20"),
//         _ => println!("Outside the range"),
//     }
// }

// fn main() {
//     let pair = (10, 20);
//     match pair {
//         (x, y) => println!("The pair is: ({}, {})", x, y),
//     }
// }

// fn main() {
//     let pair = (10, 20);
//     match pair {
//         (x, _) => println!("The first value is: {}", x),
//     }
// }

fn main() {
    let mut name = 5;
    println!("{}", name);
    name += 1;
    println!("{}", name);
}

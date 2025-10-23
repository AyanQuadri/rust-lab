enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    Changecolor(i32, i32, i32),
}

fn main() {
    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 10 };
    let msg3 = Message::Write(String::from("Hello"));
    let msg4 = Message::Changecolor(255, 0, 0);

    match msg2 {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
        Message::Write(text) => println!("Tesxt message: {}", text),
        Message::Changecolor(r, g, b) => println!("Change color to RGB! {}, {}, {}", r, g, b),
    }

    match msg1 {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
        Message::Write(text) => println!("Tesxt message: {}", text),
        Message::Changecolor(r, g, b) => println!("Change color to RGB! {}, {}, {}", r, g, b),
    }

    match msg3 {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
        Message::Write(text) => println!("Tesxt message: {}", text),
        Message::Changecolor(r, g, b) => println!("Change color to RGB! {}, {}, {}", r, g, b),
    }

    match msg4 {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
        Message::Write(text) => println!("Tesxt message: {}", text),
        Message::Changecolor(r, g, b) => println!("Change color to RGB! {}, {}, {}", r, g, b),
    }
}

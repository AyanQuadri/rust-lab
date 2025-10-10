fn main() {
    // "i8, i16, i32, i64..." these are explictly used for signed numbers ranges between (-a)->(a)
    let x: i64 = -200;
    println!("The value of x for 8 bytes is {}", x);
    // "u8, u16, u32, u64..." these are explictly used for unsigned numbers ranges from (0)->(a)
    let y: u64 = 120;
    println!("The value of y for 8 bytes is {}", y);
    // if you don't define it, it defaults with value of i32
    let z = 100;
    println!("The value of z is {z}");

    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';

    println!("decimal: {decimal}");
    println!("hex: {hex}");
    println!("octal: {octal}");
    println!("binary: {binary}");
    println!("byte: {byte}");

    // Float uses f64 by default, we can also use f32 which uses less memory
    let float1 = 5.2810182; //f64
    let float2: f32 = 2.15; //f32

    println!("float1: {float1}");
    println!("float2: {float2}");

    let t = true;
    let f: bool = false;

    println!("{t}");
    println!("{f}");

    // Character in rust
    let c = 'H';
    let character: char = '𝒽';
    let emoji = '😁';

    println!("{c}");
    println!("{character}");
    println!("{emoji}");
}

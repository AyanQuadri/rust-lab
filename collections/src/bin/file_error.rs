use std::fs::{self, File};
use std::io::ErrorKind;

fn main() {
    let file_result = File::open("src/bin/options_enum.rs");

    match file_result {
        Ok(file) => println!("File opened sucecessfully: {:?}", file),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => println!("Couldn't find the file!"),
            other_error => println!("Error opening files: {:?}", other_error),
        },
    }

    // if you just write read it will give byte data
    let read_file = fs::read_to_string("src/bin/options_enum.rs").expect("Couldn't read the file");
    println!("file: {:?}", read_file);
}

// Understanding user input in Rust
use std::io;
fn main() {
    let mut my_str = String::new();
    println!("Enter your name: ");

    io::stdin()
        .read_line(&mut my_str)
        .expect("Failed to read line");

    println!("Hello {}!", my_str);
    
}


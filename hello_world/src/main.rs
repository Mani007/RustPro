// Understanding Match in Rust

fn main() {
    let my_num = 7;

    match my_num {
        1 => println!("One"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime number"), // This will print if the value of my_num matches any of these values.
        _ => println!("Not special") // This will be printed for all other cases.
    }
}

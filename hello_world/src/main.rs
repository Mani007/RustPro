// Understanding Match in Rust

fn main() {
    let my_num = 7;
    // using match statement
    match my_num {
        x if is_even(x) => println!("Even"),
        y if !is_even(y) => println!("Odd"),        
        _ => println!("Neither even nor odd")  // default case
    }

}

fn is_even(num: u8) -> bool {
    return num%2 == 0;
 }
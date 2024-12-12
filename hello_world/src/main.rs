// Understanding Match in Rust

fn main() {
    let my_num = 7;

    match my_num {
        1 | 2 => println!("One or two"),
        3|4 => println!("Three or four"),
        5..=9 => println!("Five to nine"),
        
        
        _ => println!("Something else")  // default case
    }
}

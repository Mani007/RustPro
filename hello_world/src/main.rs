// Understanding scope in Rust
const SCOPE_CONSTANT:i32=4; // Declaring global constant, amke sure all letters are capital for global constants
fn main() {
    let outside_variable:u8 = 5;
    {
        let inside_variable:u8 = 10;
        println!("Inside variable is {}",inside_variable);
    }   
    //println!("Inside variable is {}",inside_variable); // this will create an error because the variable has been out of scope 
    println!("Outside variable is {}",outside_variable); 
    //print_out(); // this will create scope error
    println!("try to print Global Constant value {}",SCOPE_CONSTANT)
}

fn print_out(){  // if you do not use this function rust will through a warning 
    println!("try to print outside variable {}",SCOPE_CONSTANT)
}

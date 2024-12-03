// Understanding scope in Rust
fn main() {
    let outside_variable:u8 = 5;
    {
        let inside_variable:u8 = 10;
        println!("Inside variable is {}",inside_variable);
        
    }   
    println!("Outside variable is {}",outside_variable); 
}

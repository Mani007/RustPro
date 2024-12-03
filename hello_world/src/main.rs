// Understanding scope in Rust
fn main() {
    let outside_variable:u8 = 5;
    {
        let inside_variable:u8 = 10;
        println!("Inside variable is {}",inside_variable);
    }   
    //println!("Inside variable is {}",inside_variable); // this will create an error because the variable has been out of scope 
    println!("Outside variable is {}",outside_variable); 
    print_out(); // this will create scope error
}

fn print_out(){
    println!("try to print outside variable {}",outside_variable)
}

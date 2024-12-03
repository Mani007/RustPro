// Functions in Rust
fn main() {
    println!("Hello, world! Functions in Rust");
    let sum:i32 = hello_add(5,14);
    println!("Sum of the two numbers is {}",sum);
}

// Function declaration
fn hello_add(num1:i32, num2:i32) -> i32 {  // you must specify the return type in the function if required
    // code block
    println!("The functions in Rust with two numbers as input");
    let results:i32 = num1 + num2;
    return results;
}
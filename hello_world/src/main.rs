// Understanding Ownership in Rust
fn main() {
    let x:i8= 5;
    let y:&i8 = &x; // y is refereence to value of x and value of x is 5
    println!("The value of y refering to x is {}", y);
}

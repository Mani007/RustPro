// Understanding Ownership in Rust
fn main() {
    let x:i8= 50;
    let y:&i8 = &x; // y is refereence to value of x and value of x is 5
    println!("TThe value of y refering address of x is {:p}", y);
    println!("TThe address of x is at {:p}", &x);
}

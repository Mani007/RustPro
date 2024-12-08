// Understanding Ownership in Rust
fn main() {
    let x:i8= 50;
    let y:&i8 = &x; // y is refereence to value of x and value of x is 5
    println!("TThe value of y referencing address of x hence the value of x is {}", y); // rust will do auto derefereencing here and hence the value of x
    println!("TThe address of x is at {:p}", &x);
}

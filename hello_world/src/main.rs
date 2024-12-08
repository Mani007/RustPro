// Understanding Ownership in Rust
fn main() {
    let mut x:u8 = 7;
    x=x+1;
    let y = &mut x; // y is referencing to value of x. Don't read it like C/C++ as y is refering to address of x as here its refereing to value of x by auto dereferencing operator. 
    *y=*y+1;  // x=9 using dereferencing operator to increment x
    println!("The value of y ref to x is now {}", y);
    println!("The value of x is now {}", x);

}

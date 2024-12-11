// Understanding Shadowing in Rust

fn main() {
    let x = 30;
    let x = "hi";
    let x = x.len();
    println!("x is {}", x);
    println!("x is {}", x);
    


}

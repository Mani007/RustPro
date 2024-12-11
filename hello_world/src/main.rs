// Understanding Shadowing in Rust

fn main() {
    let x = 30;
    println!("x is currently as {}", x);
    let x = "hi";
    println!("x is  here as {}", x);
    let x = x.len();
    println!("x is latest here is {}", x);
   
    


}

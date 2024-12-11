// Understanding Type inference in Rust
use std::any::type_name;
fn main() {

    let x = "hello";
    let y = 3;
    let z = 4.8;

    println!("x type is {}", type_name(&x));
    println!("y type is {}", type_name(&y));
    println!("z type is {}", type_name(&z));


}

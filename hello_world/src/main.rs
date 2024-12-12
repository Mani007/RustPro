// Understanding Control Flow in Rust

fn main() {
let my_num = 12;
if my_num % 3 == 0 && my_num % 4 == 0{
    println!("my number is divisible by both 3 and 4");
} else if my_num % 3 == 0 || my_num % 4 == 0 {
println!("My number is divisible by either three or four, but not both.");
}else{
    println!("My number is not divisible by neither three nor four.");}
}

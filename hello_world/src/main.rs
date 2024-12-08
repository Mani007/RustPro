// Understanding Ownership in Rust
fn main() {
    let s1:String = String::from("HeLLo");
    let strlen:usize = calculate_len(&s1);
    println!("The lenght of string {} is {}",s1, strlen);
}
fn calculate_len(s:&String) -> usize{
    return *s.len();  // auto dereferencing happeing here
}

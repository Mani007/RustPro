// Understanding Ownership in Rust

fn main() {
    let s1:String = String::from("Hello, world!");
    let length:usize = get_string_length(s1);
    println!("The string is {} characters long.",length); // clearly this will through error as the ownership of s1 has been moved to function get_string_length
}

fn get_string_length(s:String) -> usize{
    return s.len();
}




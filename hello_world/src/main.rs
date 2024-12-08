// Understanding Ownership in Rust

fn main() {
    let s1:String = String::from("Hello, world!");
    let length:u8 = get_string_length(s1);
    println!("The string is {} characters long. and string is {}",length, s1); // clearly this will through error as the ownership of s1 has been moved to function get_string_length
}

fn get_string_length(s:String) -> u8{
    return s.len();
}




// Understanding Ownership in Rust

fn main() {
    let s1:String = String::from("Hello, world!");
    let length:usize = get_string_length(s1.clone()); // s1 ownership is not transferred but its clone is been created
    println!("The string is {} characters long for the string {}",length, s1);  // ownership of s1 is not transferred and hence s1 is still available in the main function.
}

fn get_string_length(s:String) -> usize{    // this function takes a string and but not its ownership and returns a  its length as usize type.
    let s_length:usize = s.len();
    return s_length;  // returning ownership back to caller in the form of tuple. 
}




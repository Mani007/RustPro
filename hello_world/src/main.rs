// Understanding Ownership in Rust

fn main() {
    let s1:String = String::from("Hello, world!");
    let length:usize = get_string_length(&s1); // Borrow operation using reference
    println!("The string is {} characters long for the string {}",length, s1);  // ownership of s1 is not transferred and hence s1 is still available in the main function.
}

fn get_string_length(s:&String) -> usize{    // this function takes a string reference and its ownership is not transferred to it. It just borrows the value from the calling function and returns a  its length as usize type.
    let s_length:usize = s.len();
    return s_length;  // no ownership transfer here.
}




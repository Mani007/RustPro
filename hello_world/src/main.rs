// Understanding Ownership in Rust

fn main() {
    let s1:String = String::from("Hello, world!");
    let (s2,length) = get_string_length(s1); // s1 ownership is transfered to function 
    println!("The string is {} characters long for the string {}",length, s2);  // ownership is returned in the form of s2 
}

fn get_string_length(s:String) -> (String,usize){    // this function takes a string and its ownership and returns a tuple containing the same string with its length as usize type.
    let s_length:usize = s.len();
    return (s,s_length);  // returning ownership back to caller in the form of tuple. 
}




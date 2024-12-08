// Understanding Ownership in Rust

fn main() {
    let mut s1:String = String::from("Hello, world!");
    //let length:usize = get_string_length(&s1); // Borrow operation using reference
    append_str(&mut s1);  // We try to modify original string by borrowing it and passing mutable control. 
    println!("The new string is {} ", s1);  // ownership of s1 is not transferred and hence s1 is still available in the main function.
}

fn append_str(s: &mut String) {   // We receive a mutable reference to the string which we can use to modify it.
    s.push_str(" Ya haaa!!");
}
// fn get_string_length(s:&String) -> usize{    // this function takes a string reference and its ownership is not transferred to it. It just borrows the value from the calling function and returns a  its length as usize type.
//     let s_length:usize = s.len();
//     return s_length;  // no ownership transfer here.
// }




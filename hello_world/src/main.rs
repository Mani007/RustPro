fn main() {
    println!("Hello, world! Its &str and String datatype");
    // String - Dynamic string length and heap allocated
    // &str - fixed length string and not heap allocated
    let str_literal: &str = "abcdef";  // imuutable &str type
    let string_litral1: String = String::from("abcdefgh"); // immitable String type
    let mut string_litral2: String = String::from("abcdefghijk"); // mutable string literal
    println!("the string litral is {}", str_literal);
    println!("the string litral of String type is {}", string_litral1);
    println!("the string litral of mutable string type is {}", string_litral2);
    string_litral2.push_str("lmnopqrstuvwxyz"); // adding new string to mutable String type 
    println!("the string litral of mutable string changed is {}", string_litral2);
    
}

fn main() {
    println!("Hello, world! Its integer datatype");
    let mut num:i32 = 25; // num is now mutable 
    // this will throgh a warning as num is changed before using it.
    num=12; // we can do this as num is mutable
    println!("this is num value {}", num)
    
}

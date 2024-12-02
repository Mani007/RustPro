fn main() {
    println!("Hello, world! Its integer datatype");
    let mut num:i32 = 25; // num is now mutable
    let mut num2:i32 = num; 
    // this will throgh a warning as num is changed before using it.
    println!("this is num value {}", num2);
    num=12; // we can do this as num is mutable
    println!("this is num value {}", num);
    
}

fn main() {
    println!("Hello, world! Its &str and String datatype");
    // Tuples 
    let emp_info_tuple:(&str, u16) = ("John",25);
    println!("The name of emplyee is {}, and age is {}",emp_info_tuple.0,emp_info_tuple.1);
}

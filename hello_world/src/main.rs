fn main() {
    println!("Hello, world! Its &str and String datatype");
    // Tuples 
    let emp_info_tuple:(&str, u16) = ("John",25);
    println!("The name of emplyee is {}, and age is {}",emp_info_tuple.0,emp_info_tuple.1);
    let emplyee_name: &str = emp_info_tuple.0;
    let emplyee_age: u16 = emp_info_tuple.1;
    println!("The name of employee is by tuple[0] {} and age is by tuple[1] {}",emplyee_name,emplyee_age);
    // Destructuring of tuple 
    let (name,age) = emp_info_tuple;
    println!("The name of employee is by destructuring {} and age is by destructure {}",name,age);
}

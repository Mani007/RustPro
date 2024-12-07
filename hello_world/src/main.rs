// Understanding Ownership in Rust

fn main() {
    /* This example below will work because memory for integer is stored on stack which has fixed size hence it can be copied easily. But this won't work for string data type as they are stored on heap and their size is unknown at compile time. */
    // let a:u8 = 15;
    // let b:u8 = a;
    // println!("a is {}", a);
    // println!("b is {}", b);
    /*******************/
    let str1: String = String::from("Hello"); // str1 is the pointer owing the value 'hello' or in other terms str1 is the owner of hello
    process_text(str1);  // We are transfering the ownership of str1 to process function
   // println!("The value of str1 is {}", str1) // This line will trrough error as we have transfered the ownership of str1 to the function process_text(str1) hence the error
                                              //srt1.push_str(", World!");
    //let str2: String = str1; // str2 is now pinting to str1 or you can say tranfer of ownership is happening. 'hello' is now owned by str2 now
    // This will through the borrow error as we are dealing with heap and dynamic memory using String
                             //let str2:String = srt1.clone();
    //println!("{}", str1);  // This error will occur because you have changed the ownership of the str1 to str2, but str1 does not have any owner now as its tranfered to str2
    //println!("{}", str2);
    /*
    We are dealing with heap memory here and memory is dynamic so when we assign one variable to another it will not copy the value but rather create a reference of that variable. So if you want to make a deep copy then use clone(). This concept is also known as copy trait. We will see traits in details later.
     */
}
fn process_text(s:String)  {
    println!("The string passed is {}", s)
}  // Once the value is borrowed here and the function execution is completed then the varivle s passed from the main programe will be dropped from the memory until and unless you have cloned the variable. 

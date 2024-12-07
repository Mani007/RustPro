// Understanding Ownership in Rust

fn main() {
    
   let s1:String = get_string(); // s1 gets the ownership from the function call
   println!("The value os string s1 is {}", s1);
   let s2:String = String::from("WORLD!!");   
   let s3:String = get_new_string(s2); // s2 ownership transfered to the function get_new_string
   println!("The received string of s3 is {}", s3)

}
fn get_string() -> String {
    let new_string:String = String::from("Hellloooo");   // new string variable ownership is transfered to s1 and the variable new_string in the function is dropped from the memory
    return new_string
}
fn get_new_string(recevied_string:String) -> String{  // s2 ownership is transfered here
    return recevied_string;
}


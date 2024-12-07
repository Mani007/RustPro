// Understanding Ownership in Rust

fn main() {
    
   let s1:String = get_string();
   println!("The value os string s1 is {}", s1);
   let s2:String = String::from("WORLD!!");
   let s3:String = get_new_string(s2);
   println!("The received string of s3 is {}", s3)

}
fn get_string() -> String {
    let new_string:String = String::from("Hellloooo");
    return new_string
}
fn get_new_string(recevied_string:String) -> String{
    return recevied_string;
}


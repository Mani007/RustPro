// Understanding Ownership in Rust

fn main() {
    
   let s1:String = get_string();
   println!("The value os string s1 is {}", s1);

}
fn get_string() -> String {
    let new_string:String = String::from("Hellloooo");
    return new_string
}


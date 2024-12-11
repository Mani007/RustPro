// Understanding Vectors in Rust
// Please note that unlike arrays vector is an heap allocated datatype. 
fn main() {
   let vrr:Vec<&str> = vec!["abc", "def", "ghi"];

   println!("Strings in the vector is : {:?}",vrr);

}
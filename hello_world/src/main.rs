// Understanding Vectors in Rust
fn main() {
   //let mut v:Vec<i32> = Vec::new(); // Vector declaration in rust 
   let mut v = Vec::<i32>::new();  // Vector declaration in rust

   v.push(6); // Pushing element to vector
   v.push(5); // Pushing element to vector
   v.push(4); // Pushing element to vector
   v.push(3); // Pushing element to vector
   v.push(2); // Pushing element to vector
   println!("Vector is  : {:?}",v);
   
}
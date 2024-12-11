// Understanding Vectors in Rust
fn main() {
   let mut v:Vec<i32> = Vec::new(); // Vector declaration in rust 
   v.push(5); // Pushing element to vector
   v.push(4); // Pushing element to vector
   v.push(3); // Pushing element to vector
   v.push(2); // Pushing element to vector
   println!("Vector is element at zero index : {}",v[0]);
   println!("Vector is element at one index : {}",v[1]);
   println!("Vector is element at two index : {}",v[2]);
   println!("Vector is element at three index : {}",v[3]);

}
// Understanding Vectors in Rust
fn main() {
   //let mut v:Vec<i32> = Vec::new(); // Vector declaration in rust 
   let mut v = vec![1,2,3,4,5]; // Another way to declare vector in rust
   v.push(6); // Adding elements to vector
   v.pop(); // Removing elements last index element from vector,here it will remove 6
   println!("Vector is  : {:?}",v);
   
}
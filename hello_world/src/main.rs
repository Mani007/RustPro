// Understanding Vectors in Rust
// Please note that unlike arrays vector is an heap allocated datatype. 
fn main() {
   let  vrr:Vec<&str> = vec!["abc", "def", "ghi"];   // vrr is the current owner of the vector

   write_vrr( &vrr);  // vrr ownership is transfered to the function
   println!("Strings in the vector is : {:?}",vrr); // Ownership transfered by reference and borrowed back to the caller

}
fn write_vrr(in_vrr:  &Vec<&str>) {   // in_vrr becomes the new owner of the vector

    // println!("Writing to the vector");
    // in_vrr.push("jkl");
    // in_vrr.push("mno");
    // in_vrr.push("pqr");

    // Reading the elements of a vector
    println!("Reading and printing strings from the vector : {:?}",in_vrr);
    
}
// Understanding Vectors in Rust
// Please note that unlike arrays vector is an heap allocated datatype. 
fn main() {
   let  vrr:Vec<&str> = vec!["abc", "def", "ghi"];   // vrr is the current owner of the vector

   write_vrr( vrr);
   println!("Strings in the vector is : {:?}",vrr);

}
fn write_vrr(vrr:  Vec<&str>) {
    println!("Writing to the vector");
    vrr.push("jkl");
    vrr.push("mno");
    vrr.push("pqr");

    // Reading the elements of a vector
    println!("Reading and printing strings from the vector : {:?}",vrr);
    
}
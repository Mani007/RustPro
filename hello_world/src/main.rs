// Understanding Vectors in Rust
// Please note that unlike arrays vector is an heap allocated datatype. 
fn main() {
   let mut vrr:Vec<&str> = vec!["abc", "def", "ghi"];

   write_vrr(&mut vrr);
   println!("Strings in the vector is : {:?}",vrr);

}
fn write_vrr(vrr: &mut Vec<&str>) {
    println!("Writing to the vector");
    vrr.push("jkl");
    vrr.push("mno");
    vrr.push("pqr");

    // Reading the elements of a vector
    println!("Reading and printing strings from the vector : {:?}",vrr);
    
}
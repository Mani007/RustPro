// Understanding Ownership in Rust

fn main() {
    /*Beacuse of secong mutable borrowing error, you cannot write the value of s1 in this program. */
    let mut s1:String = String::from("Hello + ");
    let write1:&mut String = &mut s1;  // write1 is borrowing s1 for writing purpose 
    println!("Value of write1 is {}", write1);
    let write2:&mut String = &mut s1;  // write2 is borrowing s1 for writing purpose 
    println!("Value of write2 is {}", write2);
    println!("The value of write1 is {} and read2 is {}", write1,write2)

}





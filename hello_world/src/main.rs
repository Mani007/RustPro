// Understanding Ownership in Rust

fn main() {
    let s1:String = String::from("Hello + ");
    let read1:&String = &s1;  // read1 is borrowing s1 for reading purpose only
    println!("Value of read1 is {}", read1);
    let read2:&String = &s1;  // read2 is borrowing s1 for reading purpose only
    println!("Value of read2 is {}", read2);
    println!("The value of read1 is {} and read2 is {}", read1,read2)

}





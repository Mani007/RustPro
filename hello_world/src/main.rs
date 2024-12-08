// Understanding Ownership in Rust

fn main() {
    /*Beacuse of secong mutable borrowing error, you cannot write the value of s1 in this program. */
    let mut s1: String = String::from("Hello + ");
    //let write1:&mut String = &mut s1;  // write1 is borrowing s1 for writing purpose
    //let write1:String = write_with1(&mut s1);
    let write1:&mut String = &mut s1;
    write1.push_str(" world");  // we are pushing value one by one hence no error
    //println!("Value of write1 is {}", write1);
    let mut read2: &mut String = &mut s1;  // write2 is borrowing s1 for writing purpose
    //let write2:String = write_with2(&mut s1);
    //let write2: &mut String = &mut s1;
    //write2.push_str(" with code."); // We are pushing one by one these values
    //println!("Value of write2 is {}", write2);
    println!("The value of read2 is {} ", read2) // The moment we call both refernce of s1, although we are just reading, the error will occour
}
// fn write_with1(s:&mut String) -> String{ // passing the borrowed reference of the string
//     let mut new_string1:String = s.push_str(" World1");
//     return new_string1;
//         // dropping the borrowed reference of the local variable s
// }
// fn write_with2(s:&mut String) -> String{  // passing the borrowed reference of the string
//     let mut new_string2:String = s.push_str(" World2");
//     return new_string2;
//     // dropping the borrowed reference of the local variable s
// }

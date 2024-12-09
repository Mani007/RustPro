// Understanding Arrays in Rust
fn main() {
    let strarr:[&str;3] = ["Hello","World","Hi!"];  // Array is created with directly passing the values of type str and length as 3.

    println!("Length of String Array:{}",strarr.len());

    for i in &strarr{
        println!("From the loop at index  and value {}",i);
    }
    println!("Element of string array {}",strarr[0]);
    println!("Element of string array {}",strarr[1]);
    println!("Element of string array {}",strarr[2]);

}

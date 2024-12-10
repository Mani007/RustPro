// Understanding Arrays in Rust
fn main() {
    let  strarr:[&str;3] = ["Hello","World","Hi!"];  // Array is created with directly passing the values of type str and length as 3.
    read_arr_ref(&strarr);  // passing array to a function by reference. 
    println!("The array elements are: {:?}",strarr);  // make sure to use the format specifier of{:?} for printing arrays.
}
fn read_arr_ref(temp_arr:&[&str;3]) {
    println!("The array elements are: {:?}",temp_arr);  // directly accessing elements of array using reference.
}
// Reading array with reference hence no mut keyword is been used
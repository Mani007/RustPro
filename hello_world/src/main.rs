// Understanding Arrays in Rust
fn main() {
    let strarr:[&str;3] = ["Hello","World","Hi!"];  // Array is created with directly passing the values of type str and length as 3.
    write_arr(strarr);
    println!("The array elements are: {:?}",strarr);  // make sure to use the format specifier of{:?} for printing arrays.
}
// The function does not lead to same result as new clone of array is getting created inside the function.
fn write_arr(mut temp_arr:[&str;3]) {  // new copy of strarr is created with name as temp_arr
    temp_arr[0] = "foolleo";
    println!(" temp_arr is {:?}",temp_arr);
}
// passing array by value is memory inefficient way of passing array to function. 
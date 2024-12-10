// Understanding Arrays in Rust
fn main() {
    let mut strarr:[&str;3] = ["Hello","World","Hi!"];  // Array is created with directly passing the values of type str and length as 3.
    write_arr(strarr);
    println!("The array elements are: {:?}",strarr);  // make sure to use the format specifier of{:?} for printing arrays.
}
fn write_arr(mut temp_arr:[&str;3]) {
    temp_arr[0] = "foolleo";
    println!(" temp_arr is {:?}",temp_arr);
}

// Understanding Arrays in Rust
fn main() {
    let mut strarr:[&str;3] = ["Hello","World","Hi!"];  // Array is created with directly passing the values of type str and length as 3.
    write_arr_ref(&mut strarr);  // passing array to a function by reference. 
    println!("The array elements are: {:?}",strarr);  // make sure to use the format specifier of{:?} for printing arrays.
}
// The function make changes in the original array of strarr as it is passed as reference to the function. 
fn write_arr_ref( temp_arr:&mut [&str;3]) {  // We are taking reference of an array here. hence we need to apply as &mut and size
    temp_arr[0] = "foolleo";
    println!(" temp_arr is {:?}",temp_arr);
}
// PLEASE GO THROUGH THE ARRAY PASSING BY VALUE AND PASSING BY REFERENCE THOROUGHLY

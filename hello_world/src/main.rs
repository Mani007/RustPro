// Understanding Arrays in Rust
fn main() {
    let a = [1,2,3];  // array type inferred as i32 and length of 3
    println!("The value of element 0 is {}",a[0]);
    let b: [i32;5] = [1,2,3,4,5]; // Array type i32 and length of 5
    println!("The value of element 0 is {}",b[0]);
    let c = [3;5]; // Array with all elements set to 3
    println!("The value of element 0 is {}",c[0]);
    let d:[u8;6] = [1,2,3,4,5,6];
    println!("The value of element 0 is {}",d[0]);
    let e = ["Hello","World"]; // String array
    println!("The value of element 0 is {}",e[0]);
    println!("The length of array d is {}",d.len());
}

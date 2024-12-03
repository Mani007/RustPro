// Understanding Ownership in Rust

fn main() {
    // let a:u8 = 15;
    // let b:u8 = a;
    // println!("a is {}", a);
    // println!("b is {}", b);
    let srt1:String = String::from("Hello");
    let str2:String = srt1;
    //let str2:String = srt1.clone();
    println!("{}",srt1);
    println!("{}",str2);
   
}


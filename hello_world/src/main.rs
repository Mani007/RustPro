// Understanding Ownership in Rust

fn main() {
    // let a:u8 = 15;
    // let b:u8 = a;
    // println!("a is {}", a);
    // println!("b is {}", b);
    let srt1:String = String::from("Hello");
    let str2:String = srt1; // This will through the borrow error as we are dealing with heap and dynamic memory using String
    //let str2:String = srt1.clone();
    println!("{}",srt1);
    println!("{}",str2);
    /*
    We are dealing with heap memory here and memory is dynamic so when we assign one variable to another it will not copy the value but rather create a reference of that variable. So if you want to make a deep copy then use clone(). This concept is also known as copy trait. We will see traits in details later. 
     */
   
}


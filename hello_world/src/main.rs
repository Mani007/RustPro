// Understanding Loops in Rust

fn main() {
let mut my_num = 12;
loop {
    if my_num == 0 {
        break; // Breaks out of the loop when condition is met.
    } else if my_num % 3 == 0 && my_num % 5 == 0 {
        println!("FizzBuzz");
    } else if my_num % 3 == 0 {
        println!("Fizz");
    } else if my_num % 5 == 0 {
        println!("Buzz");
    }

    my_num -= 1;
}
}

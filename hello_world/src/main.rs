// Understanding References in Rust
fn main() {
    let none_ref= create_str_ref(); // dangling reference as local variable cannot return reference

}
fn create_str_ref() -> &String {
    let s:String = String::from("HeLLOOO");
    return &s; // cannot return local variable s reference
}

// Hey everyone in this file we are going to understand the concept of the memory dellocation clearly in the Rust.
fn main() {
    {
        let s = String::from("Hello");
        println!("{}", s);
    } // memory for s is freed here
}
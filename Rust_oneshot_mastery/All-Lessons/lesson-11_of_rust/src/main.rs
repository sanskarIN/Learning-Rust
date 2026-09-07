// Hey everyone in this file we are going to completely understand the concept of the Memory Dellocation in the Rust:-
fn main() {
    let s = String::from("Hello");
    println!("{}", s);
} // s goes out of scope here, and its memory is freed automatically
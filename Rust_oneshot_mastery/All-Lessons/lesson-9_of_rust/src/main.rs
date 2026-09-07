// -------------------------------------------------------------
// Hey everyone today we are going to understand the concept of ownership and memory management in Rust. Ownership is a unique feature of Rust that allows for memory safety without needing a garbage collector. In this lesson, we will explore how ownership works, how to transfer ownership, and how to borrow values safely. Let's dive in and see how Rust's ownership model helps us write efficient and safe code!
// -------------------------------------------------------------
// Here is a simple example to illustrate ownership in Rust:
fn main() {
    let s1 = String::from("Hello, Rust!"); // s1 owns the String
    let s2 = s1; // Ownership of the String is moved to s2

    // println!("{}", s1); // This line would cause a compile-time error because s1 no longer owns the String

    println!("{}", s2); // This works because s2 now owns the String
}
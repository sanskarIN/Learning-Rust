// Hey everyone in thsi file we are going to understand stack vs heap in Rust.
fn main() {
    // Stack memory
    let x = 5;

    // Heap memory
    let s = String::from("Hello");

    println!("{x}");
    println!("{s}");
}
// The code ends here.
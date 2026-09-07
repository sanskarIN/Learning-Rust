// Hey everyone today in this file we are going to understand the one of the important concept of the Ownership in the Rust.
fn main() {
    let s1 = String::from("Rust");
    let s2 = s1;

    // println!("{}", s1); // error: s1 was moved
    println!("{}", s2);
}
// ------------------------------------------------------------
// The code ends here. Also see the file of the lesson-12_of_rust for the some deeper ownership concept.
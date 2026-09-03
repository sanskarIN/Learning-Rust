// ---------------------------------------------------------------------
// See PPT:- "https://docs.google.com/presentation/d/1UnPZV7K6s798RAlkanOFn1iHDK1uMqBJTnzUKYvuT2Q/edit?slide=id.g26fdac204f3_0_892#slide=id.g26fdac204f3_0_892" for more info of this crash course.
// Hey everyone in this file we are going to learn the more types of the data types in the Rust Programming Language like the usize and isize data types and also we are going to learn the size of the usize and isize data types in the Rust Programming Language.
// ---------------------------------------------------------------------
// And also we are going to learn the &str, String, char, bool, and the unit type data types in the Rust Programming Language and also we are going to learn the size of the str, string, &str, String, char, bool, and the unit type data types in the Rust Programming Language.
// ---------------------------------------------------------------------
fn main() {
    // String - Dynamic Strings
    // &str - Fixed-size string slices - Stored in the binary of the program
    // let my_string: String = String::from("Hello, world!");
    // let my_str: &str = "Hello, world!";
    // println!("String: {}", my_string);``
    let mut sentence: String = String::from("Hello Sanskar");
    sentence.push_str(" How are you?");
    println!("The sentence is: {}", sentence);
}
// ---------------------------------------------------------------------
// The code ends here.
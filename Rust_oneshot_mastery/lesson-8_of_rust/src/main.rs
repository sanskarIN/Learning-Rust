// -------------------------------------------------------
// See PPT:- "https://docs.google.com/presentation/d/1UnPZV7K6s798RAlkanOFn1iHDK1uMqBJTnzUKYvuT2Q/edit?slide=id.g26fdac204f3_0_892#slide=id.g26fdac204f3_0_892" for more info of this crash course.
// Hey everyone in this file we are going to learn about the Functions in the Rust Programming Language and also we are going to learn about the Function Parameters and Function Return Values in the Rust Programming Language.
// -------------------------------------------------------
//  Functions are a way to group code together and give it a name so that we can call it later. Functions can take parameters and return values. In Rust, functions are defined using the fn keyword, followed by the function name, a list of parameters in parentheses, and a block of code in curly braces. The return type of a function is specified after an arrow (->) following the parameter list. If a function does not return a value, it has an implicit return type of () (the unit type).
// -------------------------------------------------------
fn main() {
    // print_values();
    // I am writing the code for the adding the two variables num1 and num2 and creating the final result variable to print it.
    let num1: u8 = 10;
    let num2: u8 = 20;
    let result: u8 = add(num1, num2);
    println!("The sum of num1 and num2 is {}", result);
    // print_values(5);
}
// I am now going to write the new function.
fn add(item1: u8, item2: u8) -> u8 {
    return item1 + item2;
}

// I am now writing the code to printing the one integer value in the Rust.
// I am going to create the new function in the Rust Programming Language to and for printing the Hello Sanskar.

// fn print_values(item:u8)
// fn print_values{
// println!("Hello Sanskar")
// println!("{}",item)
// }
// -------------------------------------------------------------------
// The code ends here.
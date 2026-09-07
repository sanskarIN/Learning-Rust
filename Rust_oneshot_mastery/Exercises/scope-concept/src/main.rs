// Hey everyone today we are going to learn the concept of the scope, means the inside scope and the outside scope in the Rust.
// ------------------------------------------------------------------------------------------------------------
const CONST_GLOBAL: u8 = 10;
// As I have created the CONST_GLOBAL constant in as in the above line so, we can print anywhere in this file so, let's create the function named to be fn global and then try to print in this function and also the sam ething we can do with the variables also.
fn main() {
    let outside_variable = 10;
    {
        // Inside Scope
        let inside_variable = 20;
        // Printing inside_variable
        println!("The inside_variable is {inside_variable}");
    } //The inside_variable scope ends here after the bracket ends and trying to print to print after the ends of the bracket can lead to show the error.
      // Printing the outside_variable
    println!("The outside_variable is {outside_variable}");
    global();
    // Writing this line :- " println!("The inside_variable is {inside_variable}");" of code outside the bracket will cause to the error.
}
fn global() {
    println!("The value of the CONST_Global is {CONST_GLOBAL}")
}
// --------------------------------------------------------------------------------------------------------------
// The code ends here.
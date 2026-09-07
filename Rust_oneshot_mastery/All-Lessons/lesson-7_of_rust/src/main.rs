// ----------------------------------------------
// See PPT:- "https://docs.google.com/presentation/d/1UnPZV7K6s798RAlkanOFn1iHDK1uMqBJTnzUKYvuT2Q/edit?slide=id.g26fdac204f3_0_892#slide=id.g26fdac204f3_0_892" for more info of this crash course.
// Hey everyone in this file we are going to learn the Tupple Date-Type.
fn main() {
    // Tupple
    let emp_info:(&str,u8) = ("Pappu",32 ); // As I have created the variable named as the emp_info and named as the Pappu and his age as the 32 Years. Let us create the another variable.
    let emp_name = emp_info.0;
    let emp_age = emp_info.1;
    
    // destructuring
    let (employee_name, employee_age) = emp_info;
    println!("Employee Name={}, Employee Age={}", employee_name, employee_age);
    println!("Employee Name={}, Employee Age={}", emp_name, emp_age);
}
// ----------------------------------------------
// The code ends here. (See docs/README.md for the more information).
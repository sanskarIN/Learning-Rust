// -----------------------------------------------------------
// See PPT:- "https://docs.google.com/presentation/d/1UnPZV7K6s798RAlkanOFn1iHDK1uMqBJTnzUKYvuT2Q/edit?slide=id.g26fdac204f3_0_892#slide=id.g26fdac204f3_0_892" for more info of this crash course.
use std::mem::size_of;

fn main() {
    println!("usize: {} bytes", size_of::<usize>());
    println!("isize: {} bytes", size_of::<isize>());
}
// ---------------------------------------------------------
// The code ends here.
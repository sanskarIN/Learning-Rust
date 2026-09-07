// Hey everyoen in thsi file I am going to check the size of the some data types via writing the just Rust code.
use std::mem::size_of;

fn main() {
    println!("u8  = {} byte", size_of::<u8>());
    println!("u16 = {} bytes", size_of::<u16>());
    println!("u32 = {} bytes", size_of::<u32>());
    println!("u64 = {} bytes", size_of::<u64>());

    println!("i32 = {} bytes", size_of::<i32>());
}
// The code ends here.
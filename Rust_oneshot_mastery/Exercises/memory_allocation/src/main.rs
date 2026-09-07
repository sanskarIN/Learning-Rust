// Hey everyone! In this file, we are going to understand memory allocation in Rust.
// Rust stores values in two main places: the stack and the heap.

fn main() {
    // 1) Stack allocation
    // Integer values like i32 have a fixed size, so they are stored on the stack.
    let number: i32 = 42;
    println!("Stack value: {number}");

    // 2) Heap allocation
    // String values can change size at runtime, so they use heap memory.
    // The String stores its data in the heap and keeps a pointer on the stack.
    let name = String::from("Rust");
    println!("Heap string: {name}");

    // 3) Dynamic heap allocation with Vec
    // Vec<T> is a dynamic array that stores data on the heap.
    let mut numbers = Vec::new();
    numbers.push(10);
    numbers.push(20);
    numbers.push(30);
    println!("Vector values: {:?}", numbers);

    // 4) Automatic memory cleanup
    // Rust automatically frees memory when variables go out of scope.
    // This avoids memory leaks and dangling pointers.
    // You do not need to manually call free() or delete.
}
// --------------------------------------------------------------------
// The code ends here.
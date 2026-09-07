// Hey everyone in this file we have understand the full of Memory Allocation.
fn main() {
    // 1) Stack allocation
    // Integers like i32, u32, bool, char are stored on the stack.
    let number: i32 = 42;
    println!("Stack value: {number}");

    // 2) Heap allocation with String
    // String allocates memory on the heap because its size is not fixed at compile time.
    let name = String::from("Rust");
    println!("Heap string: {name}");

    // 3) Heap allocation with Vec
    // Vec<T> is a dynamic array stored on the heap.
    let mut numbers = Vec::new();
    numbers.push(10);
    numbers.push(20);
    numbers.push(30);

    println!("Vector values: {:?}", numbers);

    // 4) Ownership and automatic memory cleanup
    // When `name` and `numbers` go out of scope, Rust automatically frees their memory.
    // This is called RAII (Resource Acquisition Is Initialization).
    // You do not need to call free() manually.
}
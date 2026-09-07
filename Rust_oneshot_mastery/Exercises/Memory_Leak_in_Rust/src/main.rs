// The memory leak in rust is about so, little just not like in the C or C++, so let us take an example of an Rust.
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;

#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>,
}

fn main() {
    let a = Rc::new(RefCell::new(Node {
        value: 1,
        next: None,
    }));

    let b = Rc::new(RefCell::new(Node {
        value: 2,
        next: Some(a.clone()),
    }));

    a.borrow_mut().next = Some(b.clone());

    println!("A: {:?}", a);
    println!("B: {:?}", b);

    // This creates a cycle:
    // a -> b -> a
    // They keep each other alive forever.
}
// -----------------------------------------------------------------------------------
// About Memory Leaks in Rust:-
// Yes, Rust can leak memory in special cases, but normal safe code usually does not.
// The code ends here.
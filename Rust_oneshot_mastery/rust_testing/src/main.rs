/* fn main(){
    let s = String::from("Hello");
    println!("The value of the variable s is {s}")
}
The code above is succesfully tested without having the any kind of errors. */
/* fn main() {
    hello();
}

fn hello() {
    let x = 10;
    println!("The value of the variable x is {x}")
}
    The code above is runned also and tested without having any errors and bugs. */
    /* fn main() {
    static MAX: u64 = 1000;
    println!("MAX is = {}", MAX);
}
    The code above runs as well is tested without having the any errors and bugs in the code. */
    // Rust ownership move concept:-
    /* fn main () {
    let s1 = String::from("Hello");
    // If we can use this after the let s2=s1; means s2 variable then we will see the error because of the Rust ownership concept.
    println! ("Hello from variable s1 {s1}");
    let s2 = s1;
    println! ("Hello from variable s2 {s2}");
    }
    The code above is also correct. */
    // Let us the copy concept in the Rust:-
    /* fn main() {
        let x = 10;
let y = x;
println! ("The value of the x variable is {x}");
println!("The value of the y variable is {y}");
    }
    The code above is also tested and runned correctly without having the any errors. */
    // Let us try to understand the borrowing concept in the Rust:-
    /* fn use_string(s: &String) {
    println!("The value of s is {s}");
}

fn main() {
    let s = String::from("Hello");
    use_string(&s);
    println!("The value of the original variable s is {s}");
}
    The code above also runs properly. */
    // Let us understand the concept of mutable borrowing in the Rust:-
    /* fn change(s: &mut String) {
    s.push_str(", Rust!");
}

fn main() {
    let mut s = String::from("Hello");
    change(&mut s);
    println!("{}", s); // Hello, Rust!
}
    The code above is also fully runnable without having the any errors and bugs. */
    // Let us try to understand the concept of the Memory Dellocation in the Rust:-
    /* fn main() {
    let x = String::from("Learning Rust");
    println!("{x}");
} */ 
// here x is dropped and memory is deallocated
// Let us try to understand about the Memory Leaks:-
/* use std::rc::Rc;
use std::cell::RefCell;

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

    println!("Cycle created!");
    println!("a = {:?}", a);
}
    This is an example of the Memory Leak so, this does not runs properly. */
    
// Hello everyone today we are going to understand the concept of the ownership in the Rust.
// ------------------------------------------------------------------------------------------
fn main() {
    // First we are going to take this concept by taking the example of the stack and heap and then we try to undertand that how and why does it happens.
    // Stack
    let a=10;
    let b=a;
    println!("The value of a is {a} and b is {b}");
    // And when we create it by taking an examoke if heap so, we get an error so, let's see it.
    // Heap
    let str1 = String::from("Hello"); //str1 is the owner of Hello.
    // let str2 = str1; //transfer of ownership and str2 is the new owner of the Hello.
    println!("The value of the str1 is {str1}"); // and str2 is {str2} (it is being removed because this can cause the error accoring to the ownership concept.)
    // The heap example can cause the error because as we have taken the example of stack in which all the things like the storage in memory will be fixed but in the heap this is not fixed because of the dynamic data and later it can also be updated by using "str1.push_str("Sanskar");"
    // The above heap code can cause error because there is only one owner in the ownership concept but there are two owners str1 and str2 both so, this can be the main problem cause and so, because of this I caommented the code prevent the error in this line.
    // The error also comes because we tell for printing the str1 variable and now the owner becomes str2 so, this will cause the error also.
    // Or this can also be solved by commenting the line for printing the str1 and then we run so, this will run without having any errors or bugs.
// ------------------------------------------------------------------------------------------
// Let us understand the ownership via taking the more examples for it.
let y:u8= 10; // y stored in memory.
process_integer(y);
println!("The value of variable y is {y}");
// ------------------------------------------------------------------------------------------
    let x:String = String::from("Hello"); //x is the owner of Hello.
    process_string(x); // transfer of ownership.
    // println!("The value of x string is {x}");
}

fn process_integer(item1:u8) {
    println!("The value of item1 variable is {item1}") // item1 memory is 10.
    // This will clearly runs without anby error because of the stack concept.
}

fn process_string(item:String) { //item will become the new owner of the Hello so, we have to comment out for printing the line for the variable x if we don't do it so, we can get the error.
    println!("The value of itme variable is {item}");
}
// ----------------------------------------------------------------------------------------
// The code ends here.
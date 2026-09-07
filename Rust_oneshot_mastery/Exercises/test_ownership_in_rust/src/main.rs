// Hey everyone in this file we are going to test the concept of the ownership in the Rust.

fn main() {
    let s1: String = get_string(); // s1 is the owner of hello.
    println!("This is s1:{}", s1);

    let s2: String = String::from("world"); //s2 is the owner of world.
    let s3: String = send_get_string(s2); // transfer of ownership from s2 to received_string.

    println!("This is s3:{s3}"); //s3 is the new owner of world.
}

fn get_string() -> String {
    let new_string = String::from("hello"); // new_string is owner.
    return new_string; //ownership transfering.
}
// received_string owner of world.
fn send_get_string(received_string: String) -> String {
    return received_string; // transfer ownership from received_string to s3.
}
// ------------------------------------------------------------------------------------------------------------
// See PPT:- "https://docs.google.com/presentation/d/1UnPZV7K6s798RAlkanOFn1iHDK1uMqBJTnzUKYvuT2Q/edit?slide=id.g26fdac204f3_0_892#slide=id.g26fdac204f3_0_892" for more info of this crash course.
// ------------------------------------------------------------------------------------------------------------
// In this file we are going to learn for the max and min size of the data types in the rust language and this is the second file for it and in this file we are going to learn for the data types in the Rust Languge.
// Here is the deep explanation of the each line of the code and also the output of the code is also given below:-
// 1st line:- fn main() { - This is the main function of the rust language and this is the entry point of the rust program and this is the first function that is called when the rust program is executed.
// 2nd line:- println!("{}", u8::MIN); - This line is used to print the minimum value of the u8 data type and the output of this line is 0.
// 3rd line:- println!("{}", u8::MAX); - This line is used to print the maximum value of the u8 data type and the output of this line is 255.
// 4th line:- println!("{}", i8::MIN); - This line is used to print the minimum value of the i8 data type and the output of this line is -128.
// 5th line:- println!("{}", i8::MAX); - This line is used to print the maximum value of the i8 data type and the output of this line is 127.
// 6th line:- println!("u16 max = {}", u16::MAX); - This line is used to print the maximum value of the u16 data type and the output of this line is 65535.
// 7th line:- println!("u32 max = {}", u32::MAX); - This line is used to print the maximum value of the u32 data type and the output of this line is 4294967295.
// 8th line:- println!("u64 max = {}", u64::MAX); - This line is used to print the maximum value of the u64 data type and the output of this line is 18446744073709551615.
// 9th line:- println!("u128 max = {}", u128::MAX); - This line is used to print the maximum value of the u128 data type and the output of this line is 340282366920938463463374607431768211455.
// 10th line:- println!("i16 max = {}", i16::MAX); - This line is used to print the maximum value of the i16 data type and the output of this line is 32767.
// 11th line:- println!("i32 max = {}", i32::MAX); - This line is used to print the maximum value of the i32 data type and the output of this line is 2147483647.
// 12th line:- println!("i64 max = {}", i64::MAX); - This line is used to print the maximum value of the i64 data type and the output of this line is 9223372036854775807.
// 13th line:- println!("i128 max = {}", i128::MAX); - This line is used to print the maximum value of the i128 data type and the output of this line is 170141183460469231731687303715884105727.
// ------------------------------------------------------------------------------------------------------------
fn main() {
    println!("{}", u8::MIN);
    println!("{}", u8::MAX);

    println!("{}", i8::MIN);
    println!("{}", i8::MAX);
    println!("u16 max = {}", u16::MAX);
    println!("u32 max = {}", u32::MAX);
    println!("u64 max = {}", u64::MAX);
    println!("u128 max = {}", u128::MAX);

    println!("i16 max = {}", i16::MAX);
    println!("i32 max = {}", i32::MAX);
    println!("i64 max = {}", i64::MAX);
    println!("i128 max = {}", i128::MAX);
}
// The code ends here.
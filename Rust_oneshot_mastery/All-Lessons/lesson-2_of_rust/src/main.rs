// -------------------------------------------------------------
// See PPT:- "https://docs.google.com/presentation/d/1UnPZV7K6s798RAlkanOFn1iHDK1uMqBJTnzUKYvuT2Q/edit?slide=id.g26fdac204f3_0_892#slide=id.g26fdac204f3_0_892" for more info of this crash course.
// -------------------------------------------------------------
// Hey everyone we are going to learn for the rust mastery and this is the second file for it and in this file we are going to learn for the data types in the Rust Languge.
fn main() {

    let num:u8 = 255;
    println!("The value stored in num is {}",num);
    // I am creating the new variable by using the String data type means the heap type.
    let name= String::from("Sanskar");
    println!("My name is {name}");
    // I am creating the new variable of the new data of &str.
    let my_str:&str= "Sanskar";
    println!("My value stored in the my_str is {my_str}");
}
// -------------------------------------------------------------
// In this code file we also understand the concept of the variable in the Rust Programming Language.
// --------------------------------------------------------------
// The variable is a container that holds the value and the value can be changed in the variable but in the Rust Programming Language the variable is immutable by default and if we want to make it mutable then we have to use the mut keyword before the variable name.
// --------------------------------------------------------------
// The data types in the Rust Programming Language are as follows:
// 1. Integer Types: i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128
// 2. Floating Point Types: f32, f64
// 3. Boolean Type: bool
// 4. Character Type: char
// And if we don't specify the data type then the Rust Programming Language will automatically infer the data type based on the value assigned to the variable. But if we want to specify the data type then we have to use the colon (:) after the variable name and then specify the data type. And we don't specify the so, rust will automatically identify it but giving the data type becomes better because it will help the compiler to identify the data type and also it will help the programmer to understand the code better and also if we don't specify the datae type so, if we will write the big number and the data type is not specified.
// --------------------------------------------------------------
// And the formula to find the it's maximum value is 2^n - 1 and the minimum value is -2^n-1 for the signed integer types and for the unsigned integer types the maximum value is 2^n - 1 and the minimum value is 0. And the n is the number of bits used to represent the value.
//For example, if we have to find the maximum value of the i8 data type then the n is 8 and the maximum value is 2^8 - 1 = 255 and the minimum value is -2^8-1 = -128. And for the u8 data type the maximum value is 2^8 - 1 = 255 and the minimum value is 0. And for the i16 data type the maximum value is 2^16 - 1 = 65535 and the minimum value is -2^16-1 = -32768. And for the u16 data type the maximum value is 2^16 - 1 = 65535 and the minimum value is 0. And for the i32 data type the maximum value is 2^32 - 1 = 4294967295 and the minimum value is -2^32-1 = -2147483648. And for the u32 data type the maximum value is 2^32 - 1 = 4294967295 and the minimum value is 0. And for the i64 data type the maximum value is 2^64 - 1 = 18446744073709551615 and the minimum value is -2^64-1 = -9223372036854775808. And for the u64 data type the maximum value is 2^64 - 1 = 18446744073709551615 and the minimum value is 0. And for the i128 data type the maximum value is 2^128 - 1 = 340282366920938463463374607431768211455 and the minimum value is -2^128-1 = -170141183460469231731687303715884105728. And for the u128 data type the maximum value is 2^128 - 1 = 340282366920938463463374607431768211455 and the minimum value is 0.
// And the size of the data type is as follows:
// 1. i8: 8 bits
// 2. i16: 16 bits
// 3. i32: 32 bits
// 4. i64: 64 bits
// 5. i128: 128 bits
// 6. isize: depends on the architecture (32 bits on 32-bit systems, 64 bits on 64-bit systems)
// 7. u8: 8 bits
// 8. u16: 16 bits
// 9. u32: 32 bits
// 10. u64: 64 bits
// 11. u128: 128 bits
// 12. f32: 32 bits
// 13. f64: 64 bits
// 14. bool: 1 bit
// 15. char: 4 bytes (32 bits)
// Means the maximum value of the i8 is 127 and the minimum value is -128 and the maximum value of the u8 is 255 and the minimum value is 0. And the maximum value of the i16 is 32767 and the minimum value is -32768 and the maximum value of the u16 is 65535 and the minimum value is 0. And the maximum value of the i32 is 2147483647 and the minimum value is -2147483648 and the maximum value of the u32 is 4294967295 and the minimum value is 0. And the maximum value of the i64 is 9223372036854775807 and the minimum value is -9223372036854775808 and the maximum value of the u64 is 18446744073709551615 and the minimum value is 0. And the maximum value of the i128 is 170141183460469231731687303715884105727 and the minimum value is -170141183460469231731687303715884105728 and the maximum value of the u128 is 340282366920938463463374607431768211455 and the minimum value is 0. And for f32, it can represent values in a range from approximately -3.4 x 10^38 to 3.4 x 10^38, with a precision of about 6-7 decimal digits. For f64, it can represent values in a range from approximately -1.7 x 10^308 to 1.7 x 10^308, with a precision of about 15-16 decimal digits. The bool type can have two possible values: true or false. The char type represents a single Unicode character, which can be any valid Unicode scalar value (a code point) ranging from U+0000 to U+D7FF or U+E000 to U+10FFFF.
//--------------------------------------------------------------
// The code ends here. See docs/README.md for the more information.
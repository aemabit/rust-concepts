/*
 * Rust check the data types when compile the code. (static type)
*/

fn main() {
    // Integer
    let signed_integer: i8 = 23;
    println!("Signed integer is: {}", signed_integer);

    // Unsigend integer can be negative.
    let unsigned_integer: u8 = 40;
    println!("Unsigned integer is: {}", unsigned_integer);

    // Integer literals
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    println!(
        "Literals are: \n decimal: {} \n hex: {} \n octal: {} \n binary: {}",
        decimal, hex, octal, binary
    );

    // Float
    let float = 5.0;
    let float32: f32 = 12.432;
    println!("Float are: \n f64: {} \n f32: {}", float, float32);

    // Boolean
    let true_variable = true;
    let false_variable = false;
    println!(
        "Boolean are: \n true: {} \n false: {}",
        true_variable, false_variable
    );

    // Character
    let character = 'a';
    let emoji = '😙';
    println!("Character are: \n true: {} \n false: {}", character, emoji);

    // Compound Types

    // Tuples
    let tuple = ("h", 23, -45, 0.22, '😙');
    let declared_tupla: (&str, u32, i32, f32, char) = ("h", 23, -45, 0.22, '😙');
    println!("declared_tupla are: {:?}", declared_tupla);

    let (v, w, x, y, z) = tuple;
    println!("Values are\n v{}, w:{}, x:{}, y:{}, z:{}", &v, &w, &x, &y, &z);
    println!(
        "or values are\n  v{}, w:{}, x:{}, y:{}, z:{}",
        tuple.0, tuple.1, tuple.2, tuple.3, tuple.4
    );

    // Array
    let array_of_integers = [1, 2, 3, 4, 5];
    println!("Array Value in slot 3 = {}", array_of_integers[2]);
    // [i32;4] = i32 data type inside of array | 4 are the range of the array
    let declared_array: [&str;4] = ["Hello", "World", "in", "array"];
    println!("Declared array value in slot 1 = {}", declared_array[0]);

    // Strings
    let name: &str = "Andres";
    println!("Name is: {}", name);
    let last_name: String = "Mejias".to_string();
    println!("Last Name is: {}", last_name);
}   

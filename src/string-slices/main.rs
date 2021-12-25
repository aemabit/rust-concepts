// STRING & STRING SLICE    
fn main() {
    // &str -> string slide
    let hello = "Hello!";

    // lib std -> 
    let name_string = String::from("Andres");

    // hardcode binary -> point to memory slot 
    let name_str = "Eduardo"; 

    // transfor &str to String
    let mut name_str_to_string = name_str.to_string();
    name_str_to_string.push('a');

    let name_string_to_str = &name_string[..4];

    println!("{}", hello);
    println!("name string: {}", name_str_to_string);
    println!("name str: {}", name_string_to_str); 


}

use std::{fs::File};
use std::io::{Error, ErrorKind};

// HANDING ERROR
/*
Exist two types of errors,
- recoverable: sample -> when open a file ant the path is incorrect.
- non-recoverable: sample -> when try to access beyond its limit.
*/

fn main() {

    // panic! (macro) is a non-recoverable error
    // to get the verbose use `RUST_BACKTRACE=1 cargo run`
    // panic!("Explode");

    let file = File::open("some/path/bad");
    match file {
        Ok(file)=> read_file(file),
        Err(error) => println!("Error: {:?}", error.kind())
    }

    // USE ERROR KIND
    // match file {
    //     Ok(file)=> read_file(file),
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => println!("file not found"),
    //         _ => println!("unknow error"),
    //     }
    // }

    // let file_two = File::open("other/path").unwrap();
    let file_third = File::open("some/path").expect("The same as unwrap but with a custom text");


}

fn read_file(file: File){
    println!("here")
}
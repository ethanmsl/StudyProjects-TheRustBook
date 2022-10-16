// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

use std::fs::File;
use std::io::{self, ErrorKind, Read};

#[allow(unused_variables)]
fn main() {
    // // the assignment will succeed even if the file isn't found, but the
    // // variable's value will be of type Error
    // let greeting_file_result = File::open("hello.txt");
    //
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };

// ------------- handling different errors differently -------------
    // let greeting_file_result = File::open("hello.txt");
    //
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         // if 'not found' error
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //                                          // ^ try to create it and...
    //             Ok(fc) => fc, 
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file {:?}", other_error);
    //         }
    //     },
    // };


// ------------- .unwrap & .expect -------------
    // // .unwrap() is a shortcut for the above T or panic! match
    // let greeting_file = File::open("hello.txt").unwrap();

    // // .expect("...") allows you to give more info and context in you error message
    // let greeting_file_result = File::open("hello.txt")
    //     .expect("hello.txt should be included in this project");


}



#[allow(dead_code)]
// ------------- Propagating errors, verbose -------------
fn read_username_from_file_verbose() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
        //          ^ we *return* the error here
        //            skipping the rest of the logic which is for translating
        //            a file into a string
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}


#[allow(dead_code)]
// ------------- Propagating errors, '?' operator -------------
// '?': same as the first match expresion above
//    if Ok(t) => t
//    if Err(e) => return Err(e)
//  DIFFERENCE: the '?' operator will use 'from' to convert the returned error
//              to the error type of return in the function signature
fn read_username_from_file_quest_op() -> Result<String, io::Error> {
    let mut username_file= File::open("hello.txt")?;
    //                                            ^ '?'
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    //                                         ^ '?'

    Ok(username)
}


#[allow(dead_code)]
// ------------- Propagating errors, '?' operator w/ chaining -------------
fn read_username_from_file_chaining() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

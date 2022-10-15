// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

use std::fs::File;

#[allow(unused_variables)]
fn main() {
    // the assignment will succeed even if the file isn't found, but the
    // variable's value will be of type Error
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}

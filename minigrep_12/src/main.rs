use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    // ^ note: the first argument will be the name of the program that collects the arguments
    // e.g., in this case, "target/debug/minigrep_12"
    //                       ^ NOTE: I'm not sure why "target" is there, probably
    //                               something to do with where the binary is kept
    dbg!(&args);
    // args = dbg!(args);
    // QUESTION: ^ args is immutable, if I give it's value away is ther anyway
    //             for it to get ownership back??

    // ignore calling_program = &args[0];
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

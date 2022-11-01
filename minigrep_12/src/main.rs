use std::env;
use std::fs;

// args = dbg!(args);
// QUESTION: ^ args is immutable, if I give it's value away is ther anyway
//             for it to get ownership back??

fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, file_path) = parse_config(&args);

    println!();
    println!("Searching for {}", query);
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!();
    println!("With text:\n{contents}");
}

fn parse_config(args: &[String]) -> (&str, &str) {
    // ignore calling_program = &args[0];
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}

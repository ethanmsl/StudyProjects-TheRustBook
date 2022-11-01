use std::env;
use std::fs;

// args = dbg!(args);
// QUESTION: ^ args is immutable, if I give it's value away is ther anyway
//             for it to get ownership back??

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!();
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!();
    println!("With text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        // Guardian //
        if args.len() <3 {
            panic!("not enough arguments");
        }

        // Passed Guardian //
        // ignore calling_program = &args[0];
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}


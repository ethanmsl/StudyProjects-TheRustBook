use std::env;
use std::process;

use minigrep_12::Config;

// args = dbg!(args);
// QUESTION: ^ args is immutable, if I give it's value away is ther anyway
//             for it to get ownership back??

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep_12::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

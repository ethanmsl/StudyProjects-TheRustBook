use std::env;
use std::process;

use minigrep_improved_13::Config;

// args = dbg!(args);
// QUESTION: ^ args is immutable, if I give it's value away is ther anyway
//             for it to get ownership back??

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep_improved_13::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

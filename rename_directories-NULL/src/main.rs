use std::env;

use rename_directories_NULL as lib;

fn main() -> std::io::Result<()> {
    // local path (test)
    let path_prepend = String::from(".");
    // // intended use path (one-off prod)
    // let path_prepend = String::from("/Users/eskowronski-lutz/Documents/Programming_Langs/Rust/book-projects-rust");
    let args_iterator = env::args();

    lib::run(args_iterator, path_prepend)
}

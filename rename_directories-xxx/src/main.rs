use std::env;
use std::fs;

use rename_directories_xxx as lib;

fn main() -> std::io::Result<()> {
    // local path (test)
    let path_prepend = String::from(".");
    // // intended use path (one-off prod)
    // let path_prepend = String::from("/Users/eskowronski-lutz/Documents/Programming_Langs/Rust/book-projects-rust");
    let args_iterator = env::args();
    let files_iterator = fs::read_dir(&path_prepend)?;
    lib::files_print_swaps(files_iterator);
    // lib::run(args_iterator, path_prepend)
    Ok(())
}

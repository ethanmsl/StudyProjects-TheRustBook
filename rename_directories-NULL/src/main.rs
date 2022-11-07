use std::env;
use std::fs;

use rename_directories_NULL::ToFromPair;

fn main() -> std::io::Result<()> {
    // local path (test)
    let path_prepend = String::from(".");
    // // intended use path (one-off prod)
    // let path_prepend = String::from("/Users/eskowronski-lutz/Documents/Programming_Langs/Rust/book-projects-rust");

    let args_iterator = env::args();
    let to_from = ToFromPair::from_args(args_iterator, &path_prepend);

    println!("to: {:?}", to_from);

    fs::rename(to_from.to, to_from.from)?;
    Ok(())
}

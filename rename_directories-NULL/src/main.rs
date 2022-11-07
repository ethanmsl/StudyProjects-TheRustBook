use std::env;
use std::fs;

use rename_directories_NULL as lib;
use rename_directories_NULL::ToFromPair;

fn main() -> std::io::Result<()> {
    // local path (test)
    let path_prepend = String::from(".");
    // // intended use path (one-off prod)
    // let path_prepend = String::from("/Users/eskowronski-lutz/Documents/Programming_Langs/Rust/book-projects-rust");
    let args_iterator = env::args();
    let arg_length = args_iterator.len();

    match arg_length {
        3 => {
                let to_from = ToFromPair::from_args(args_iterator, &path_prepend);
                println!("to: {:?}", to_from);
                fs::rename(to_from.from, to_from.to,)?;
        },
        2 => {
            let mut args_iterator = args_iterator;
            let to_arg = args_iterator.last().unwrap();
            // println!("args_it...: {}", args_iterator.last().unwrap());
            let swapped_name = lib::swap_dashes_and_underscores(&to_arg);
            println!("swapped_name: {}", swapped_name);

            let to_from = ToFromPair::new(to_arg, swapped_name);
            println!("tofrom: {:?}", to_from);
        },
        _ => println!("Please provide two arguments: the directory to rename and the new name for the directory."),
    }
    Ok(())
}

use std::env;
use std::fs;

use rename_directories_xxx as lib;
use lib::test_prints::print_wave;

fn main() -> std::io::Result<()> {
    // exploring how to call objects with modularized code
    lib::test_prints::print_hello(); // because can call 'lib'
    lib::print_bye();                // because lib has a use statement bringing in 'print_bye()'
    print_wave();                    // becuase we have a use statment from 'lib::...' in this file
    // test_prints::print_bye();        // Not Valid
    // print_bye();                     // Not Valid


    // get args, first should be path, if second we'll see if it's a path
    let args_iterator = env::args();
    // let maybe_path = lib::check_for_path_argument(args_iterator);
    //
    //
    // local path (test)
    let path_prepend = String::from(".");
    // // intended use path (one-off prod)
    // let path_prepend = String::from("/Users/eskowronski-lutz/Documents/Programming_Langs/Rust/book-projects-rust");

    let files_iterator = fs::read_dir(&path_prepend)?;
    lib::files_print_swaps(files_iterator);
    // lib::run(args_iterator, path_prepend)
    Ok(())
}

use std::env;
use std::fs;

use lib::arg_parse;
use lib::test_prints::print_wave;
use rename_directories_xxx as lib;

fn main() -> std::io::Result<()> {
    // //////////////////////// test prints ////////////////////////
    // exploring how to call objects with modularized code
    lib::test_prints::print_hello(); // because can call 'lib'
    lib::print_bye();                // because lib has a use statement bringing in 'print_bye()'
    print_wave();                    // becuase we have a use statment from 'lib::...' in this file
    // test_prints::print_bye();     // Not Valid
    // print_bye();                  // Not Valid
    // ////////////////////////////////////////////////////////////

    // //////////////////////// arg parse ////////////////////////
    // get args, first should be path, if second we'll see if it's a path
    let args_iterator = env::args();
    let maybe_path = arg_parse::check_for_path_argument(args_iterator);
    let path_prepend = if let Some(path) = maybe_path {
        println!("Path: {}", path);
        path
    } else {
        println!("No path provided");
        String::from(".")
    };
    dbg!(&path_prepend);
    // ////////////////////////////////////////////////////////////
    // TODO: argument-given path can have a '/' in it at the end
    //       need to sanitize it
    // ////////////////////////////////////////////////////////////

    let files_iterator = fs::read_dir(&path_prepend)?;
    lib::files_print_swaps(files_iterator);
    // lib::run(args_iterator, path_prepend)
    Ok(())
}

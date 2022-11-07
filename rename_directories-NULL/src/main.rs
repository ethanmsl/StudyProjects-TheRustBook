use std::env;
use std::fs;
use std::fs::rename;

use rename_directories_NULL as lib;

fn main() -> std::io::Result<()> {
    // local path (test)
    let path_prepend = String::from(".");
    // // intended use path (one-off prod)
    // let path_prepend = String::from("/Users/eskowronski-lutz/Documents/Programming_Langs/Rust/book-projects-rust");
    let args_iterator = env::args();
    let files: Vec<_> = 
        fs::read_dir(&path_prepend)?
        .map(|path| path.unwrap().path())
        .collect();
    let files_iterator = fs::read_dir(&path_prepend)?;
    for file in files_iterator {
        let file = file.unwrap().path();
        let file_ref = file.to_str().unwrap();
        println!("file o.: {:?}", file_ref);
        let changed = lib::swap_dashes_and_underscores(file_ref);
        println!("changed: {:?}", changed);

    }
    
    // if files[3].to_str() == Some("./a.txt") {
    //     println!("matched 'a.txt'");
    //     fs::rename(&files[3], &files[3].with_file_name("b.txt"))?;
    // } else {
    //     println!("did not match 'a.txt'");
    // }

    // lib::run(args_iterator, path_prepend)
    Ok(())
}

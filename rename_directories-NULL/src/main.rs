use std::fs;
use std::env;

#[derive(Debug)]
struct ToFromPair {
    to: String,
    from: String,
}

impl ToFromPair {
    fn new(to: String, from: String) -> ToFromPair {
        ToFromPair { to, from }
    }

    fn from_args(mut args: impl Iterator<Item = String>, prepend: &str) -> ToFromPair {
        let curr_dir = args.next().unwrap();
        println!("curr_dir: {}", curr_dir);
        println!("prepend: {}", prepend);

        let to_arg   = format!("{}/{}", prepend,args.next().unwrap());
        let from_arg = format!("{}/{}", prepend,args.next().unwrap());

        ToFromPair::new(to_arg, from_arg)
    }
}

    // if let Err(e) = minigrep_12::run(config) {
    //     eprintln!("Application error: {e}");
    //     process::exit(1);
    // }

fn main() -> std::io::Result<()> {
    let args_iterator = env::args();
    let path_prepend = String::from("/Users/eskowronski-lutz/Documents/Programming_Langs/Rust/book-projects-rust");
    let to_from = ToFromPair::from_args(args_iterator, &path_prepend);

    println!("to: {:?}", to_from);

    // fs::rename("/Users/eskowronski-lutz/Documents/Programming_Langs/Rust/book-projects-rust/boop.txt", "peeb.txt")?;
    Ok(())
}

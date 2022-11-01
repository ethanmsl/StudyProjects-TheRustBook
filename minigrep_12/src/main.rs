use std::env;
use std::fs;
use std::process;
use std::error::Error;

// args = dbg!(args);
// QUESTION: ^ args is immutable, if I give it's value away is ther anyway
//             for it to get ownership back??

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!();
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // if let <pattern> = ... { }
    //         ^ just like patterns in a Match statement, can bind variables
    // NOTE: we use 'if let' vs 'unwrap_or_else' because we're not interested
    // in the output of run() [which is side-effect only]
    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
    // // Question: what was the scope of the variable bound in the above expression
    // // Answer: V(via below) - it apepars to just be within the brackets of the if let statement
    // println!("Application error: {e}");
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!();
    println!("With text:\n{contents}");

    Ok(())
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        // Guardian //
        if args.len() <3 {
            return Err("not enough arguments");
        }

        // Passed Guardian(s) //
        // ignore calling_program = &args[0];
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}


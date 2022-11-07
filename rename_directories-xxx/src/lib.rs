use std::fs;

#[derive(Debug)]
pub struct ToFromPair {
    pub from: String,
    pub to:   String,
}

impl ToFromPair {
    fn new(from: String, to: String) -> ToFromPair {
        ToFromPair { from, to }
    }

    pub fn from_args(mut args: impl Iterator<Item = String>, prepend: &str) -> ToFromPair {
        let curr_dir = args.next().unwrap();
        println!("curr_dir: {}", curr_dir);
        println!("prepend: {}", prepend);

        let from_arg = format!("{}/{}", prepend, args.next().unwrap());
        let to_arg   = format!("{}/{}", prepend, args.next().unwrap());

        ToFromPair::new(from_arg, to_arg)
    }
}

pub fn swap_dashes_and_underscores(input: &str) -> String {
    input
        .chars()
        .map(|c| match c {
            '-' => '_',
            '_' => '-',
            _ => c,
        })
        .collect()
}

pub fn files_print_swaps(files_iterator: impl Iterator<Item = std::io::Result<fs::DirEntry>>) {
    for file in files_iterator {
        let file = file.unwrap().path();
        let file_ref = file.to_str().unwrap();
        println!("file o.: {:?}", file_ref);
        let changed = swap_dashes_and_underscores(file_ref);
        println!("changed: {:?}", changed);
    }
}

// NOTE: this is not easily testable
//       perhaps the main structure should be broken up for testing
//       ... I'm really not sure how best to deal with this...
pub fn run<T: ExactSizeIterator + Iterator<Item = String>>(
    args_iterator: T,
    path_prepend: String,
) -> std::io::Result<()> {
    let arg_length = args_iterator.len();

    match arg_length {
        3 => {
            let to_from = ToFromPair::from_args(args_iterator, &path_prepend);
            println!("to: {:?}", to_from);
            fs::rename(to_from.from, to_from.to)?;
        }
        2 => {
            let to_arg = args_iterator.last().unwrap();
            // println!("args_it...: {}", args_iterator.last().unwrap());
            let swapped_name = swap_dashes_and_underscores(&to_arg);
            println!("swapped_name: {}", swapped_name);

            let to_from = ToFromPair::new(to_arg, swapped_name);
            println!("tofrom: {:?}", to_from);

            fs::rename(to_from.from, to_from.to)?;
        }
        _ => eprintln!("Either a single path or two paths is expected & required."),
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_swaps() {
        let input    = String::from("hello-world and -_---_-");
        let expected = String::from("hello_world and _-___-_");
        let actual = swap_dashes_and_underscores(&input);
        assert_eq!(expected, actual);
    }

    // #[test]
    // fn to_from_pair_from_args() {
    //     let fake_args = vec![String::from("path"), String::from("--a--b__c.boop")];
    //     run(fake_args.into_iter(), String::from("."));
    //     // assert_eq!(expected, actual);
    // }
}

// NOTE: this allows 'print_bye' to be used without path call besides 'lib::'
//       in parent caller (main.rs)
pub mod test_prints;
pub use test_prints::print_bye;

pub mod to_from;
pub use to_from::ToFromPair;

pub mod arg_parse;

pub use string_change::*;
pub mod string_change {
    use std::fs;

    fn swap_dashes_and_underscores(input: &str) -> String {
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

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn line_swaps() {
            let input = String::from("hello-world and -_---_-");
            let expected = String::from("hello_world and _-___-_");
            let actual = swap_dashes_and_underscores(&input);
            assert_eq!(expected, actual);
        }
    }
}

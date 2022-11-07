#[derive(Debug)]
pub struct ToFromPair {
    pub from: String,
    pub to: String,
}

impl ToFromPair {
    pub fn new(from: String, to: String,) -> ToFromPair {
        ToFromPair { from, to }
    }

    pub fn from_args(mut args: impl Iterator<Item = String>, prepend: &str) -> ToFromPair {
        let curr_dir = args.next().unwrap();
        println!("curr_dir: {}", curr_dir);
        println!("prepend: {}", prepend);

        let from_arg = format!("{}/{}", prepend, args.next().unwrap());
        let to_arg = format!("{}/{}", prepend, args.next().unwrap());

        ToFromPair::new(from_arg, to_arg,)
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

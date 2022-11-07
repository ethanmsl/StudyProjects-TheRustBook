#[derive(Debug)]
pub struct ToFromPair {
    pub to: String,
    pub from: String,
}

impl ToFromPair {
    fn new(to: String, from: String) -> ToFromPair {
        ToFromPair { to, from }
    }

    pub fn from_args(mut args: impl Iterator<Item = String>, prepend: &str) -> ToFromPair {
        let curr_dir = args.next().unwrap();
        println!("curr_dir: {}", curr_dir);
        println!("prepend: {}", prepend);

        let to_arg = format!("{}/{}", prepend, args.next().unwrap());
        let from_arg = format!("{}/{}", prepend, args.next().unwrap());

        ToFromPair::new(to_arg, from_arg)
    }
}

pub fn swap_dashes_and_underscores(input: String) -> String {
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
        let actual = swap_dashes_and_underscores(input);
        assert_eq!(expected, actual);
    }
}

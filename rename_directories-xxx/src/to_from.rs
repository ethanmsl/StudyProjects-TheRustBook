
#[derive(Debug)]
pub struct ToFromPair {
    pub from: String,
    pub to:   String,
}

impl ToFromPair {
    pub fn new(from: String, to: String) -> ToFromPair {
        ToFromPair { from, to }
    }

    pub fn from_args(mut args: impl Iterator<Item = String>, prepend: &str) -> ToFromPair {
        // Guardian 1
        if prepend.ends_with('/') {
            panic!("prepend should not end with a '/'\nthe code assumes this came from a sanitized input");
        }

        // diagnostic
        let curr_dir = args.next().unwrap();
        println!("curr_dir: {}", curr_dir);
        println!("prepend: {}", prepend);

        // extract
        let from_raw = args.next().unwrap();
        let to_raw = args.next().unwrap();

        // Guardian 2
        if from_raw.starts_with('/') || to_raw.starts_with('/') {
            panic!("neither from_raw nor to_raw.to should begin with a '/'");
        }
        let from_arg = format!("{}/{}", prepend, from_raw);
        let to_arg   = format!("{}/{}", prepend, to_raw);

        // additional guardianship of path formatting could be added here
        // however this function is probably already deprecated, so I likely won't
        ToFromPair::new(from_arg, to_arg)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_from_pair() {
        let from = String::from("from");
        let to   = String::from("to");
        let pair_struct = ToFromPair::new(from, to);
        assert_eq!(pair_struct.from, "from");
        assert_eq!(pair_struct.to, "to");
    }

    #[test]
    fn to_from_pair_from_args() {
        let args = vec!["curr_dir".to_string(), "from".to_string(), "to".to_string()];
        let pair_struct = ToFromPair::from_args(args.into_iter(), "prepend");
        assert_eq!(pair_struct.from, "prepend/from");
        assert_eq!(pair_struct.to, "prepend/to");
    }


    #[test]
    #[should_panic(expected = "prepend")]
    fn to_from_pair_from_args_prepend_broken() {
        let args = vec!["curr_dir".to_string(), "from".to_string(), "to".to_string()];
        ToFromPair::from_args(args.into_iter(), "prepend/");
    }

    #[test]
    #[should_panic(expected = "neither from_raw nor to_raw")]
    fn to_from_pair_from_args_from_broken() {
        let args = vec!["curr_dir".to_string(), "/from".to_string(), "to".to_string()];
        ToFromPair::from_args(args.into_iter(), "prepend");
    }

    #[test]
    #[should_panic(expected = "neither from_raw nor to_raw")]
    fn to_from_pair_from_args_to_broken() {
        let args = vec!["curr_dir".to_string(), "from".to_string(), "/to".to_string()];
        ToFromPair::from_args(args.into_iter(), "prepend");
    }
}

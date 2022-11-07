#[derive(Debug)]
pub struct ToFromPair {
    pub to:   String,
    pub from: String,
}

impl ToFromPair {
    fn new(to: String, from: String) -> ToFromPair {
        ToFromPair { to, from }
    }

    pub fn from_args(mut args: impl Iterator<Item = String>, prepend: &str) -> ToFromPair {
        let curr_dir = args.next().unwrap();
        println!("curr_dir: {}", curr_dir);
        println!("prepend: {}",  prepend);

        let to_arg   = format!("{}/{}", prepend, args.next().unwrap());
        let from_arg = format!("{}/{}", prepend, args.next().unwrap());

        ToFromPair::new(to_arg, from_arg)
    }
}


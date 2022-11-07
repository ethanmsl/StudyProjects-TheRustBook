
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
        let curr_dir = args.next().unwrap();
        println!("curr_dir: {}", curr_dir);
        println!("prepend: {}", prepend);

        let from_arg = format!("{}/{}", prepend, args.next().unwrap());
        let to_arg   = format!("{}/{}", prepend, args.next().unwrap());

        ToFromPair::new(from_arg, to_arg)
    }
}

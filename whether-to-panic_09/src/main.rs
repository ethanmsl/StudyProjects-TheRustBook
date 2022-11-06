
use std::net::IpAddr;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
     // if value < 1 || value > 100 {
        if !(1..=100).contains(&value) {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

#[allow(unused_imports)]
#[allow(unused_variables)]
fn main() {
    // -------- expect/unwrap for compiler-opaque non-errorable code --------
    //          Use expect("..") to document reason for beleiving non-errorable
    //          Helpful for debugging (if incorrect), reading code, and
    //          recognizing changes needed if changing that area of code
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
}

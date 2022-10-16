
use std::net::IpAddr;

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

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // ^ note: the first argument will be the name of the program that collects the arguments
    // e.g., in this case, "target/debug/minigrep_12"
    //                       ^ NOTE: I'm not sure why "target" is there, probably
    //                               something to do with where the binary is kept
    dbg!(args);
}

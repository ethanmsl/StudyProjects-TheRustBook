//prevent error warnings
// #![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
// #![allow(dead_code)]
fn main() {
    let mut s = String::new();

    // (1)
    let data = "initial contents";
    let s = data.to_string();

    // (2) the method also works on a literal directly:
    let s = "initial contents".to_string();

    // (3)
    let s = String::from("initial contents");
}

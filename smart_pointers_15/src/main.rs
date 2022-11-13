//! Ch. 15. Smart Pointers - "The Rust Programming Language"

mod recursive;

/// the function that serves as insertion to run
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
    let b = Box::new(5.);
    println!("b = {}", b);
    let b: Box<u8> = Box::new(5);
    println!("b = {}", b);
}

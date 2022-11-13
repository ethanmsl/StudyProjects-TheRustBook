//! Ch. 15. Smart Pointers - "The Rust Programming Language"

mod recursive;
pub use recursive::List::{Cons, Nil};

/// the function that serves as insertion to run
fn main() {

    // /////// Recursive Type with Box /////// //
    let rec_list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // ////// Basic Box Syntax ////// //
    let b = Box::new(5);
    println!("b = {}", b);
    let b = Box::new(5.);
    println!("b = {}", b);
    let b: Box<u8> = Box::new(5);
    println!("b = {}", b);
}

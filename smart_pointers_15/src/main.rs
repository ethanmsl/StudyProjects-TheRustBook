//! Ch. 15. Smart Pointers - "The Rust Programming Language"

mod recursive;
pub use recursive::List::{Cons, Nil};

///

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;
impl<T> Deref for MyBox<T> {
    type Target = T;
    // NOTE: `-> &Self` not `-> &self` (capital 'S')
    // capital-`Self` references the TYPE
    // lowercase-`self` is synsugar for `param: Self`
    //     sugar mapping:
    //     //    self       ~=>~ self: Self
    //     //    &self      ~=>~ self: &Self
    //     //    &mut self  ~=>~ self: &mut Self
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
struct MyBox<T>(T);

/// the function that serves as insertion to run
fn main() {
    // /////// Deref'ing /////// //
    {
        let x = 5;
        let y = &x;
        // the asserts will panic on normal run if not-true
        assert_eq!(5, x);
        assert_eq!(5, *y);

        let z = *y;
        println!("z: {}, y: {}, x: {}", z, y, x);

        let x = 6;
        let y = x;
        let z = y;
        println!("z: {}, y: {}, x: {}", z, y, x);
        // ^ because of copy trait

        // try with something on the heap
        let x = String::from("hello");
        let y = &x;
        let z = y;
        println!("z: {}, y: {}, x: {}", z, y, x);

        let x = String::from("hello");
        let y = &x;
        let z = &y;
        let a = *z;
        // let b = *a; => String
        // let c = *b; => str
        // let d = *c; => xxx

        println!("z: {}, y: {}, x: {}", z, y, x);

        // let a = String::from("wowowwow");
        // let b = *a;

        // now with boxes
        let x = 5;
        let y = Box::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    // /////// Recursive Type with Box /////// //
    let _rec_list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // ////// Basic Box Syntax ////// //
    let b = Box::new(5);
    println!("b = {}", b);
    let b = Box::new(5.);
    println!("b = {}", b);
    let b: Box<u8> = Box::new(5);
    println!("b = {}", b);
}

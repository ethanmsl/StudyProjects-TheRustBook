//! Ch. 15. Smart Pointers - "The Rust Programming Language"
#![allow(unused_variables)]

mod customsmartptr;
mod recursive;
pub use customsmartptr::CustomSmartPointer;
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
    // ^ NOTE: `*` will BOTH run the above deref and then do a standard deref of the
    //         returned value -- hence:
    //         *(MyBox(beep)) ~~> *(MyBox(beep).deref()) ~~> *(&beep) ~~> beep
    //         If we returned the value itself then we'd be taking ownership
    //         away from the object, which is rarely what we'd want -- so this ends up
    //         being ergonomic (we we to get around it -- which we probably wouldn't
    //         want to as it would be unexpected deref behavior) -- I don't know if we
    //         could nicely do that via standard impl methods... (mostly curious as to
    //         the allowances and means of working with the system)
}
struct MyBox<T>(T);

pub enum ListRc {
    ConsR(i32, Rc<ListRc>),
    Nilly,
}
use std::rc::Rc;

/// the function that serves as insertion to run
fn main() {
    // /////// Rc<t> : Multi-Ownership /////// //
    {
        use crate::ListRc::{ConsR, Nilly};
        let a = Rc::new(ConsR(5, Rc::new(ConsR(10, Rc::new(Nilly)))));
        println!("count after creating a = {}", Rc::strong_count(&a));
        let b = ConsR(3, Rc::clone(&a));
        println!("count after creating a = {}", Rc::strong_count(&a));
        {
            let c = ConsR(4, Rc::clone(&a));
            println!("count after creating a = {}", Rc::strong_count(&a));
        }
        println!("count after creating a = {}", Rc::strong_count(&a));
        drop(b);
        println!("count after creating a = {}", Rc::strong_count(&a));
    }

    // /////// Custom Drop in a Custom Smart Pointer /////// //
    {
        let c = CustomSmartPointer {
            data: String::from("c: my stuff"),
        };
        let d = CustomSmartPointer {
            data: String::from("d: other stuff"),
        };
    }
    let e = CustomSmartPointer {
        data: String::from("e: yet more stuff"),
    };
    let f = CustomSmartPointer {
        data: String::from("f: something I'm going to call `drop()` on"),
    };
    println!("CustomSmartPointers created.");

    drop(f);
    println!("CustomSmartPointer `f` dropped before the end of main.");

    // /////// Deref Coercion /////// //
    {
        fn hello(name: &str) {
            println!("Hello, {}!", name);
        }

        let myboxed_string = MyBox::new(String::from("Rust"));
        hello(&myboxed_string);
        // ^ ¡QUELLE SURPRISE!
        // Rust will **dereference as many times as 'needed' to get a matching type!
        // (presumably these deref chains are always linear (?) ... yeah)
    }

    // /////// Deref'ing /////// //
    {
        let x = 5;
        let y = MyBox::new(x);
        assert_eq!(5, x);
        // ^ I think this works because i32 implements Copy
        assert_eq!(5, *y);

        let x = String::from("five");
        assert_eq!("five", x);
        if "five" == x {
            println!("x is five");
        }
        // Okay so even though "five" is probably a "string literal"
        // it seems to "==" a string with the same char sequence

        // let x = String::from("five");
        // let y = MyBox::new(x);
        // assert_eq!("five", x);
        // // ^ Yep! the value was now borrowed! :)
        // assert_eq!("five", *y);
    }

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

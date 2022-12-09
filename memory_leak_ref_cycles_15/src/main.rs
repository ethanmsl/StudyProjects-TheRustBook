use crate::List::{Cons, Nil};  // Huh, so importing locally
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    println!("-----------------------------\n");

    let a = Rc::new( Cons(  5, RefCell::new(Rc::new(Nil)) ) );

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());
    println!("-----------------------------\n");

    let b = Rc::new( Cons( 10, RefCell::new(Rc::clone(&a)) ) );

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());
}

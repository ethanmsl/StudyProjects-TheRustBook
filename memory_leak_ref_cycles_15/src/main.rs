use crate::List::{Cons, Nil}; // Huh, so importing locally
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

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Debug)]
enum Muskateer {
    Athos,
    Porthos,
    Aramis,
}

impl Distribution<Muskateer> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Muskateer {
        match rng.gen_range(0..=2) {
            0 => Muskateer::Athos,
            1 => Muskateer::Porthos,
            _ => Muskateer::Aramis,
            // ^ hmm, gives me guff if I have that as '2', seems like it can't tell
            // that the generated o-domain is only {0,1,2} / perhaps there's something
            // about the code *I* don't understand
        }
    }
}

fn main() {
    {
        // Three Muskateers Enum w/ Random & Static assignment
        println!("-----------------------------\n");

        let m_static = Muskateer::Porthos;
        let m_rand = rand::random::<Muskateer>();

        println!("Muskateer \"m_static\" is {:?}", m_static);
        println!("Muskateer \"m_rand\" is {:?}", m_rand);
    }

    {
        // Reference Cycle --> Memory Leak
        println!("-----------------------------\n");

        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

        println!("a initial rc count = {}", Rc::strong_count(&a));
        println!("a next item = {:?}", a.tail());
        // ^ interesting that `tail` is defined for `List`, but works for
        // `Rc<List>`; also it prints out value of `Nil`, not `Rc(Nil)`
        // Clearly the judo/movement of pointers & smart pointers is something I
        // still must learn more about (re: basics/norms)
        println!("-----------------------------\n");

        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
        //                                    ^ note: this is an *Rc*::clone

        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count = {}", Rc::strong_count(&b));
        println!("b next item = {:?}", b.tail());
        println!("-----------------------------\n");

        let link = a.tail();
        // ^ this seems to work fine, so why were we treating `a.tail()` 
        // as an option below?

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("b rc count after changing a = {}", Rc::strong_count(&b));
        println!("a rc count after changing a = {}", Rc::strong_count(&a));
        println!("-----------------------------\n");

        // Uncomment the next line to see that we have a cycle;
        // it will overflow the stack
        // println!("a next item = {:?}", a.tail());
        //   ^ lol
    }
}

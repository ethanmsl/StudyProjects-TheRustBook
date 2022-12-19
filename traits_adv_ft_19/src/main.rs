//! Advanced Traits -- Ch 19 Advanced Features

use std::fmt::Display;

/// Same as `Iterator`; here to look at syntax
pub trait IteratorBespoke {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

/// a one field struct
pub struct Counter {
    count: u32,
}

impl Counter {
    /// create a new, zero-valued `Counter`
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

/// I don't think this gets added to docs (the comment, that is)
/// This is a *twisted*, almost malicious implementation of `next` (copilot suggested)
/// Interesting to consider whether it would properly be acceptable...
/// I suppose... so long as the only assumption is that things that use it have
/// the contracts agreed upon ... then yes
impl IteratorBespoke for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        //                         ^ this is a bit confusing tome
        //          as it's really just whatever type 'item' is which isn't related to,
        //          in this case, IteratorBespoke or Counter.
        //          I *think* the best way to think about it is
        //          as `Item` belongs to the trait-type (`IteratorBespoke`)
        //          , the whole thing being replaced by the concrete type supplied
        self.count += 1;

        // l-o-f'ing-l
        if self.count < 6 {
            None
        } else {
            Some(self.count)
        }
    }
}

fn main() {
    println!("---------------------------------------------\n");
    // `IteratorBespoke` code
    {
        let mut cntr = Counter::new();
        for _ in 0..12 {
            match cntr.next() {
                Some(x) => println!("cntr: {}", x),
                None => println!("cntr: None, too small"),
            }
        }
    }
    println!("---------------------------------------------\n");

    // Default Type Params & Overloading
    {
        use std::ops::Add;

        #[derive(Debug, Copy, Clone, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }

        impl Point {
            fn new(x: i32, y: i32) -> Self {
                Point { x, y }
            }
        }

        impl Add for Point {
            type Output = Point;

            fn add(self, other: Point) -> Point {
                Point::new(self.x + other.x, self.y + other.y)
            }
        }
        let p1 = Point::new(1, 0);
        let p2 = Point::new(2, 3);
        let p3 = p1.add(p2);
        dbg!(p3);
        let p4 = p1 + p3;
        dbg!(p4);

        impl Add<i32> for Point {
            type Output = Point;

            fn add(self, other: i32) -> Point {
                Point::new(self.x + other, self.y + other)
            }
        }

        let p1 = Point::new(1, 0);
        let p3 = p1.add(10);
        dbg!(p3);
        let p4 = p3 + 5;
        dbg!(p4);
        // let p5 = 5 + p3;
        // dbg!(p5);
        // // no impl for i32+Point
    }
    println!("---------------------------------------------\n");

    // disambiguation syntax
    {
        struct Human;

        trait Pilot {
            fn fly(&self);
            fn smile();
        }

        trait Wizard {
            fn fly(&self);
            fn smile();
        }

        impl Human {
            fn fly(&self) {
                println!("*waving arms furiously*");
            }

            fn smile() {
                println!("*smiles*");
            }
        }

        impl Pilot for Human {
            fn fly(&self) {
                println!("This is your captain speaking.");
            }
            fn smile() {
                println!("*smiles broadly*");
            }
        }

        impl Wizard for Human {
            fn fly(&self) {
                println!("Up!");
            }
            fn smile() {
                println!("*winks*");
            }
        }

        let person = Human;
        person.fly();
        Pilot::fly(&person);
        Wizard::fly(&person);

        Human::smile();
        <Human as Pilot>::smile();
        <Human as Wizard>::smile();
        // There could be multiple implementaitons of the Pilot or Wizard traits
        // for various types -- we need to specify the sub- & trait- types together
    }
    println!("---------------------------------------------\n");

    // "super traits" which are "sub-trait-types of other trait-types"
    {
        use std::fmt;

        trait OutlinePrint: fmt::Display {
            // a default implementation is provided
            fn outline_print(&self) {
                let output = self.to_string();
                let len = output.len();
                println!("{}", "*".repeat(len + 4));
                println!("*{}*", " ".repeat(len + 2));
                println!("* {} *", output);
                println!("*{}*", " ".repeat(len + 2));
                println!("{}", "*".repeat(len + 4));
            }
        }

        struct Point {
            x: i32,
            y: i32,
        }

        impl fmt::Display for Point {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "({}, {})", self.x, self.y)
            }
        }

        impl OutlinePrint for Point {}

        let p = Point { x: 72, y: 296 };
        p.outline_print();

        println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n");

        // NewType pattern and foundling rule
        // NOTE: no runtime penalty for this -- wrapper type is elided as part of compile
        struct Wrapper<D: fmt::Display>(Vec<D>);

        impl<D> fmt::Display for Wrapper<D>
        where
            D: fmt::Display,
        {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    f,
                    "[{}]",
                    self.0
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
        }

        impl<D> OutlinePrint for Wrapper<D> where D: fmt::Display {}

        let w = Wrapper(vec![1, 2, 3, 4, 5, 6, 7]);
        println!("w = {}", w);
        w.outline_print();
    }
    println!("---------------------------------------------\n");
}

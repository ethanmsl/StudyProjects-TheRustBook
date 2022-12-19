//! Advanced Traits -- Ch 19 Advanced Features

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
    let mut cntr = Counter::new();
    for _ in 0..12 {
        match cntr.next() {
            Some(x) => println!("cntr: {}", x),
            None => println!("cntr: None, too small"),
        }
    }
}

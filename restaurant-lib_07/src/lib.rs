// // included code from 'cargo new <...> --lib'
// // like the 'hello world' code that plain 'cargo new <...>' creates
// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

use std::fmt::Result;
use std::io::Result as IoResult;
//                  ^ 'as'

use std::{cmp::Ordering, io};
//       ^ "nested paths"
// use std::io::{self, Write};
//             ^ defines 'use std::io'
use std::collections::*;
//                    ^ brings everything into name space
//                    often used to import tests ('...test::*')
//                    ^ only items at that level or also all their children
//                      and paths to their children...?

// this brings our module into scope
mod moving_to_another_file;

//pub use crate::moving_to_another_file;

mod front_of_house {
    // 'pub mod' allows ancestor modules to refer to it, but does not provide
    // access to its member functions
    // I'm not sure what the purpose of non-pub modules would be -- a module
    // where you couldn't access it or anything within it?  ... to what purpose?
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn server_order() {}

        fn take_payment() {}
    }
}

// can also declare 'pub use ...' and... I think that makes the aliasing
// essentially work outside of the scope it's declared in...
// I'm not yet sure ... sounds like this will be revisited at least in 
// Ch14, so confident I'll have some examples to work it out with
use crate::front_of_house::hosting;

pub fn whatever_name() {
    // making use of the 'use' declaration above
    // MUST be in same scope as a 'use' statement enabling access to 'hosting'
    hosting::add_to_waitlist();
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // uses 'super' keyword to refer to parent module
        super::deliver_order();
    }

    fn cook_order() {}


    // even though *struct* is public
    // it's fields are variable public/private
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // this public function gives 'outsiders' a way to set the private field
    impl Breakfast {
        pub fn summer (toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // if an *enum* is made public
    // all of it's potential values are as well
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restauraunt() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    // we can use both of these value variants from 
    // our Appetizer *enum* becuase we set the enum to public
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

// -------------------------------

fn deliver_order() {}



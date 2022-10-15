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

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // uses 'super' keyword to refer to parent module
        super::deliver_order();
    }

    fn cook_order() {}


    // even though struct is public
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

    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

// -------------------------------

fn deliver_order() {}



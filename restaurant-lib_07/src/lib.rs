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


pub fn eat_at_restauraunt() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

// -------------------------------

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // uses 'super' keyword to refer to parent module
        super::deliver_order();
    }

    fn cook_order() {}
}

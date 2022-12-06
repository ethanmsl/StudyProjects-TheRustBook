#![allow(unused_variables)]
#![allow(dead_code)]

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    // Working Messenger implementation
    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    // // Panic-inducing Messenger implementation
    // // Can be compiled because RefCell<T> does borrow-checking at runtime
    // impl Messenger for MockMessenger {
    //     fn send(&self, message: &str) {
    //         let mut one_borrow = self.sent_messages.borrow_mut();
    //         let mut two_borrow = self.sent_messages.borrow_mut();
    //         // ^ Panicks at second co-mutable assignment
    //
    //         one_borrow.push(String::from(message));
    //         // two_borrow.push(String::from(message));
    //         // ^ even without use the above will panic
    //         // (which turns out to be how compilation mutability works too
    //         // I was thinking it would check for use -- I think that may
    //         // onlybe in the case re: looking via immutable ref after mutating)
    //         //  ^ which I need to check how RefCell<T> handles
    //     }
    // }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }

    #[test]
    fn verifying_compiler_looks_at_mutable_reference_usage() {
        let mut vev = vec![1, 2, 3];
        let mut vev_1 = &mut vev;
        // let mut vev_2 = &mut vev;
        // ^ this does prevent compilation even if not used

        let mut boop = vec![3, 2, 1];
        vev_1 = &mut boop;
        assert_eq!(*vev_1, vec![3, 2, 1]);

        // this does not require vev_1 to be mutable
        // vev_1.push(4);
        // assert_eq!(vev, vec![1,2,3,4]);

        // this does not require vev_1 to be mutable
        // vev_1[2] = 5;
        // assert_eq!(vev, vec![1,2,5]);
    }
}

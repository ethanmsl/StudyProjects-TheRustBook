//! shared_state_concurrency_16
//!
//! NOTE: I need to look up **Deadlocking** dangers of `Mutex`es
//! also NOTE: I need to look into more primtive `atomic` types in general
//!            and the `std` and `core` atomic libraries
//!            (also need to determine relationship of `std` & `core`)

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("-------------------------------------------------\n");
    {
        let m = Mutex::new(5);

        {
            let mut num = m.lock().expect("\nMutex got poisoned\n");
            //               ^ *aquire*_lock / lock(_everyone_else_out)
            *num += 1;
            println!("m = {:?}", m);
            //             ^ won't be able to access value of mutex
            //           as mutex is still 'locked'
            //           (though it *will* print a value ... "<locked>", which is nice)
        }
        // ^ mutex-lock drops due to falling out of scope

        println!("m = {:?}", m);
    }
    println!("-------------------------------------------------\n");

    {
        let counter = Arc::new(Mutex::new(0));
        //            ^ new type! :)
        //              Arc is 'Atomic_Rc' (atomic reference couting)
        //              'atomic's are primitives that share well across threads
        //              We wrap the mutex in it here so that we can share ownership
        //              of the Mutex across threads
        //              (note: Rc is read only, so I'm not sure how this is all gonna
        //               play out...)
        //               ^ answer: `Mutex` provides/has *interior mutability*
        //                 so this works much like `RefCell` + `Rc` does
        let mut handles = vec![];
        // ^ vector of thread handles, to join to main later

        for _ in 0..10 {
            //   0..=9  <-- still thinking about comparative values of practices 
            //              of range notation...
            let counter = Arc::clone(&counter);
            //  ^ nice example of idiomatic use of shadowing in Rust!!
            //    we make 'counter' temporarily mean an intentionally adulterated
            //    version of counter -- allowing us to keep the following code the same
            //    (for better and worse, I suppose) and maintaining, perhaps, the core
            //    semantic ideas of the variable in context (almost like reverse module
            //    name scoping -- keeping a simple name in a local context where meaning
            //    would be clear, though, outside that context the additional machiner
            //    that we've added via the shadow assignment would be quite relevant)
            //    -- and the shadowed variables is nicely & conveniently dropped
            //       at the end of the shadow's birth scope
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();

                *num += 1;
                // ^ unlike `RefCell` there's no explicit `.borrow_mut()` method
                //   invoked in order to write to the de-referenced value / to use
                //   the interior mutability of the smart-pointer
            });
            handles.push(handle);
        }

        // Joining all the threads, who's handles were stored in a vector
        for handle in handles {
            handle.join().unwrap();
        }
        println!("Result: {}", *counter.lock().unwrap());
    }
    println!("-------------------------------------------------\n");
}

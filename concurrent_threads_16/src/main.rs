//! 16.1 -- Using Threads to Run Code Simultaneously

use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..=20 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(2));
        //                                  ^ can play with to affect threads
        //                                    from above that occur
    }
}

//! 16.1 -- Using Threads to Run Code Simultaneously

use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..=9 {
            println!("- spawned thread: loop number {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("main thread: loop number {}", i);
        thread::sleep(Duration::from_millis(1));
        //                                  ^ can play with to affect threads
        //                                    from above that occur

    // NOTE: with both sleep-times equal the number of loops printed by the
    //       spawned thread is variable - as is the precise order of the two
    //       prints relative to one another
    }
}

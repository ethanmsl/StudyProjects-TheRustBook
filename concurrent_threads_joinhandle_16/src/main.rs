//! concurrent threads -- now with `JoinHandle`s
//! `JoinHandle`: struct that allows us to use `.join()` to join thread
//! into another -- amounting, I imagine, to effectively ``await thread_x completion``

use std::thread;
use std::time::Duration;

fn main() {
    let handle_a = thread::spawn(|| {
        for i in 1..10 {
            println!("- spawn-thrd--A--: loop number ##{}##", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    let handle_b = thread::spawn(|| {
        for i in 1..10 {
            println!("- spawn-thrd--B--: loop number @ {} @", i);
            thread::sleep(Duration::from_millis(1));
        }
        handle_a.join().unwrap();
        // ^ joining thread_a to thread_b
    });

    for i in 1..5 {
        println!("main thread: loop number {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    let _boop = handle_b.join();
    //  ^               ^ joinging thread_b, which has thread_a joined to it
    //  ^ a Result is returned - partly just `()`, but also something new...
}

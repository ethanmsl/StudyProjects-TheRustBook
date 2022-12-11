//! concurrent threads -- now with `JoinHandle`s

use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("- spawned thread: loop number {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
            println!("main thread: loop number {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
    // ^ so, I think, `handle.join()` is basically ``await thread_x completion``
}

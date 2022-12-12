//! Message Passing to trasfer Data Between Threads
//!

use std::sync::mpsc;
//              ^ multi-producer, single-consumer
use std::thread;
use std::time::Duration;
// import sleep
use std::thread::sleep;

fn main() {
    println!("--------------------------------------------\n");
    {
        let (tx, rx) = mpsc::channel();

        // Spawned thread, which communicates via the tx channel defined above
        thread::spawn(move || {
            for i in 0..10 {
                println!("- spawned: loop #{}", i);
                sleep(Duration::from_millis(100));
                // ^ sleeping, so slow
            }

            let val = String::from("hi");
            tx.send(val).unwrap();
            // ^ NOTE: the last two lines disambiguate the type of the `let (tx, rx)`
            //         declaration above
            // ^ also NOTE: that `tx` is *moved* into this closure
        });

        // let received = rx.recv().unwrap();
        // println!("`rx` got the following message: {}", received);
        // // ^ even if commented out the entire spawned thread's slow loop prints out
        // //   before the main threads fast loop
        // //   so presumably the main thread is 'blocked' (right term?) wiating for rx...
        // //   By Contrast:
        // //                If we comment out both received (and the line using it)
        // //                Then main loop prints fully mostly or wholly pre-empting the
        // //                spawned thread's loop

        for i in 0..10 {
            println!("main: loop #{}", i);
            // no sleep, so fast
        }

        // sleep(Duration::from_millis(1200));
        // // ^ can turn on or off to allow the spanwed thread to finish
        if let Ok(received) = rx.try_recv() {
            println!(
                "`rx` got the following message wraped in `Ok()`: {}",
                received
            );
        } else {
            println!("`rx` did not receive an `Ok()` wrapped-result");
        };
        // ^ returns a `Result`, and is non-blocking
    }

    // {
    //     // Example of trying to use a value after it is send via .send(...)`
    //     // causes a compile-time error
    //     // `.send(...)` takes ownership of the value
    //     let (tx, rx) = mpsc::channel();
    //
    //     thread::spawn(move || {
    //         let val = String::from("hi");
    //         tx.send(val).unwrap();
    //         println!("val is {}", val);
    //     });
    //
    //     let received = rx.recv().unwrap();
    //     println!("`rx` got the following message: {}", received);
    // }
}

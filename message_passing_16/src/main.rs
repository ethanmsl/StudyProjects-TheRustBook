//! Message Passing to trasfer Data Between Threads
//!

use std::sync::mpsc;
//              ^ multi-producer, single-consumer
use std::thread;

fn main() {
    println!("--------------------------------------------\n");
    {
        let (tx, rx) = mpsc::channel();

        // Spawned thread, which communicates via the tx channel defined above
        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
            // ^ NOTE: the last two lines disambiguate the type of the `let (tx, rx)`
            //         declaration above
            // ^ also NOTE: that `tx` is *moved* into this closure
        });

        let received = rx.recv().unwrap();
        println!("`rx` got the following message: {}", received);
    }
}

//! 16.1 -- Using Threads to Run Code Simultaneously

use std::thread;
use std::time::Duration;

fn main() {
    {
        // Sleep-ensured thread swap opportunities
        println!("-------------------------------\n");
        thread::spawn(|| {
            for i in 1..=9 {
                println!("- spawned thread: loop number {}", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        // thread::sleep(Duration::from_millis(1));
        for i in 1..4 {
            println!("main thread: loop number {}", i);
            thread::sleep(Duration::from_millis(1));
            //                                  ^ can play with to affect threads
            //                                    from above that occur
        }
        // NOTE: with both sleep-times equal the number of loops printed by the
        //       spawned thread is variable - as is the precise order of the two
        //       prints relative to one another
    }

    {
        // Many threads with nor forced sleep time
        println!("-------------------------------\n");
        thread::spawn(|| {
            for i in 1..=50 {
                print!("-sp{}-", i);
            }
        });

        // thread::sleep(Duration::from_millis(1));
        for i in 1..50 {
            print!("*{}*", i);
        }
        // NOTE: with enough repetitions you'll still see the 'spawned' thread
        //       print as well, *sometimes*
        // NOTE_2: it also ends with a strange block char of `%` when it ends on
        //         a spawned char -- is something being interruptd mid-print?
        println!();
    }

    println!("-------------------------------\n");
}

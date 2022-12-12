//! shared_state_concurrency_16
//!

use std::sync::Mutex;
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
}

//! lib.rs file
//! currently contains our `ThreadPool` implementation

use std::{
    fmt,
    sync::{mpsc, Arc, Mutex},
    thread,
};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

/// Errors that can occur when creating a new `ThreadPool`.
/// (currently only one kind: mis-initialization with zero threads)
#[derive(Debug, Clone)]
pub enum PoolCreationError {
    InvalidThreadCount,
}

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to create ThreadPool")
    }
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// NOTE: `new` should be guaranteed to work, so this is
    ///       poor use of convention; but I'm running with our book here
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        // assert!(size > 0);
        // // ^ NOTE: there should be a way of insisting that these logical checks
        // //         are done **at compile time**
        // //         (that could probably be done with a macro hmmm ... would there be a
        // //          nice way to get the compiler to work with us on that ...)
        // ThreadPool

        Self::build(size).unwrap()
        // ^ just here to use the build function below
        //   (mostly for kicks / to check that it works)
    }

    /// implicitly errorable creation "build" (vs implicitly guaranteed creation "new")
    /// returns a specific `PoolCreationError` if called with invalid argument
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size == 0 {
            return Err(PoolCreationError::InvalidThreadCount);
        }

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Ok(ThreadPool { workers, sender })
    }

    // using the signature of `Thread::spawn` as a guide
    // but without planning to return anything or rejoin the thread
    // except as a empty thread that prompts possible connection completion
    // (so 'mostly' empty)
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
        //                     ^ only applies if the `send()` fails
        //                       (and then returns its contents)
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            //                          ^ in particular may result in panic
            //                            if mutex is poisoned
            //                            e.g. due to a previous holder panicking
            //                            (thus a holder of mutex-lock panicking would
            //                             result in a panick domino effect)

            println!("--Worker {} got a job; executing.--", id);

            job();
            // ^ run the closure we received via the mutexed-receiver-end of channel
        });

        Worker { id, thread }
    }
}

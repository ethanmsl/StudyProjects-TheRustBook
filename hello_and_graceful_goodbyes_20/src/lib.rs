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
    sender: Option<mpsc::Sender<Job>>,
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
    /// Build a new ThreadPool.
    /// implicitly errorable creation "build" (vs implicitly guaranteed creation "new")
    /// returns a specific `PoolCreationError` if called with invalid argument
    ///
    /// The size is the number of threads in the pool.
    ///
    /// NOTE: `new` should be guaranteed to work, so this is
    ///       poor use of convention; but I'm running with our book here
    ///
    /// # Panics
    ///
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

        Ok(ThreadPool {
            workers,
            sender: Some(sender),
        })
    }

    /// delivers a closure to the ThreadPool to be executed by a worker thread
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender
            .as_ref()
            .unwrap()
            .send(job)
            .expect("receiver unavailable (likley dropped)");
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        // |               ^ moves sender from ThreadPool and then gives it to drop
        // |                 while leaving a `None` in its place
        // |                 (so we can shut down possible sends first and then take
        // |                  care of shutting down the workers)
        // ^ closes the mpsc-channel, which will error out the `recv` calls in
        //   all of the threads' `loop`s

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                //                               ^ feels a but wonkadoo
                //                                 a workaround to wanting ownership
                //                                 , but working through a ref.
                //                                 Seems like there ought to have been
                //                                 a way to just take ownership of the
                //                                 worker struct itself
                thread.join().expect("couldn't join on associated thread");
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().expect("mutex likely poisoned").recv();

            match message {
                Ok(job) => {
                    println!("--Worker {} got a job; executing.--", id);
                    job();
                }
                Err(_) => {
                    println!(
                        "Worker {} received shutdown signal or otherwise errored out.",
                        id
                    );
                    break;
                    // would be nicer to better pass a message that causes graceful closure
                    // rather than causing an error that could plausibly represent
                    // multiple histories/causes
                }
            };
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

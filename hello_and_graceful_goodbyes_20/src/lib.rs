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

        Ok(ThreadPool { workers, sender })
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
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

            println!("--Worker {} got a job; executing.--", id);

            job();
        });

        Worker { id, thread }
    }
}

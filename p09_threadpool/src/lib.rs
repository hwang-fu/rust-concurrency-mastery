use std::{
    sync::{Arc, Mutex, mpsc},
    thread::{self, JoinHandle},
};

/// A job is a boxed closure that runs once and can be sent across threads.
type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

struct Worker {
    id: usize,
    handle: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let handle = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv();
                match message {
                    Ok(job) => {
                        println!("Worker {id}: executing job");
                        job();
                    }
                    Err(_) => {
                        // Channel closed, time to shut down
                        println!("Worker {id}: shutting down");
                        break;
                    }
                }
            }
        });

        let handle = Some(handle);

        Worker { id, handle }
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0, "ThreadPool size must be greater than 0");

        let (sender, receiver) = mpsc::channel();
        let sender = Some(sender);
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap()
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Drop the sender to close the channel
        // This signals workers to shut down
        drop(self.sender.take());

        // Wait for all workers to finish
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(handle) = worker.handle.take() {
                handle.join().unwrap();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        sync::atomic::{AtomicUsize, Ordering},
        time::Duration,
    };

    use super::*;

    #[test]
    fn test_basic_execution() {
        let pool = ThreadPool::new(4);
        let counter = Arc::new(AtomicUsize::new(0));

        for _ in 0..8 {
            let cloned_counter = Arc::clone(&counter);
            pool.execute(move || {
                cloned_counter.fetch_add(1, Ordering::SeqCst);
            });
        }

        // Give jobs time to complete
        thread::sleep(Duration::from_millis(100));

        drop(pool);

        assert_eq!(counter.load(Ordering::SeqCst), 8);
    }

    #[test]
    fn test_100_jobs_across_4_workers() {
        let pool = ThreadPool::new(4);
        let counter = Arc::new(AtomicUsize::new(0));

        for _ in 1..=100 {
            let counter_clone = Arc::clone(&counter);
            pool.execute(move || {
                counter_clone.fetch_add(1, Ordering::SeqCst);
            });
        }

        drop(pool); // Wait for all jobs to complete

        assert_eq!(counter.load(Ordering::SeqCst), 100);
        println!("All 100 jobs completed!");
    }

    #[test]
    fn test_graceful_shutdown() {
        let pool = ThreadPool::new(2);
        let flag = Arc::new(AtomicUsize::new(0));

        let flag_clone = Arc::clone(&flag);
        pool.execute(move || {
            thread::sleep(Duration::from_millis(50));
            flag_clone.store(1, Ordering::SeqCst);
        });

        // Pool drops here, should wait for job to complete
        drop(pool);

        // Job should have completed before drop returned
        assert_eq!(flag.load(Ordering::SeqCst), 1);
    }

    #[test]
    #[should_panic(expected = "ThreadPool size must be greater than 0")]
    fn test_zero_size_panics() {
        let _pool = ThreadPool::new(0);
    }
}

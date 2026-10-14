// thread_pool.rs (challenge) - solution
//
// `let message = receiver.lock().unwrap().recv();` holds the lock only
// for that one statement: the MutexGuard is a temporary, dropped at the
// semicolon, so other workers can receive while this one runs its job.
// (`while let Ok(job) = receiver.lock().unwrap().recv()` would keep the
// guard alive for the whole loop body, so only one job runs at a time.)
//
// The Sender lives in an Option so `drop` can `take()` it. Dropping it
// closes the channel; workers drain what's left, see `Err`, and exit.
// Only then do the `join`s return.

use std::sync::{Arc, Mutex, mpsc};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

struct ThreadPool {
    workers: Vec<thread::JoinHandle<()>>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0, "a thread pool needs at least one thread");

        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));

        let workers = (0..size)
            .map(|_| {
                let receiver = Arc::clone(&receiver);
                thread::spawn(move || {
                    loop {
                        let message = receiver.lock().unwrap().recv();
                        match message {
                            Ok(job) => job(),
                            Err(_) => break, // channel closed: shut down
                        }
                    }
                })
            })
            .collect();

        ThreadPool { workers, sender: Some(sender) }
    }

    fn size(&self) -> usize {
        self.workers.len()
    }

    fn execute<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender
            .as_ref()
            .expect("sender only goes away in drop")
            .send(Box::new(job))
            .expect("workers are alive while the pool exists");
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in self.workers.drain(..) {
            worker.join().unwrap();
        }
    }
}

fn main() {
    let pool = ThreadPool::new(3);
    for id in 0..6 {
        pool.execute(move || println!("job {id} ran on {:?}", thread::current().id()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::{Duration, Instant};

    #[test]
    fn reports_size() {
        assert_eq!(ThreadPool::new(3).size(), 3);
    }

    #[test]
    #[should_panic]
    fn zero_threads_panics() {
        ThreadPool::new(0);
    }

    #[test]
    fn runs_every_job_before_drop_returns() {
        let counter = Arc::new(AtomicUsize::new(0));
        let pool = ThreadPool::new(4);
        for _ in 0..50 {
            let counter = Arc::clone(&counter);
            pool.execute(move || {
                thread::sleep(Duration::from_millis(5));
                counter.fetch_add(1, Ordering::SeqCst);
            });
        }
        drop(pool); // must wait for all queued jobs to finish
        assert_eq!(counter.load(Ordering::SeqCst), 50);
    }

    #[test]
    fn jobs_run_in_parallel() {
        let pool = ThreadPool::new(4);
        let start = Instant::now();
        for _ in 0..4 {
            pool.execute(|| thread::sleep(Duration::from_millis(300)));
        }
        drop(pool);
        // One at a time would take 1200ms; four at once takes ~300ms.
        assert!(start.elapsed() < Duration::from_millis(900), "took {:?}", start.elapsed());
    }

    #[test]
    fn results_can_come_back_through_a_channel() {
        let pool = ThreadPool::new(2);
        let (tx, rx) = mpsc::channel();
        for n in 1..=5u64 {
            let tx = tx.clone();
            pool.execute(move || tx.send(n * n).unwrap());
        }
        drop(tx);
        let mut results: Vec<u64> = rx.iter().collect();
        results.sort();
        assert_eq!(results, [1, 4, 9, 16, 25]);
    }
}

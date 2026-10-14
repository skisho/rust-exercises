// thread_pool.rs (challenge)
//
// Spawning a thread per task is expensive. A thread pool starts a fixed
// number of worker threads once, then hands them jobs through a queue.
// This is how web servers and job runners keep many tasks moving.
//
// Design:
//   - `ThreadPool::new(n)` starts n workers (and panics if n is 0).
//   - Jobs are boxed closures sent down an `mpsc` channel.
//   - Every worker loops: take the next job from the channel and run it.
//     A channel has only one Receiver, so the workers share it through
//     `Arc<Mutex<Receiver<Job>>>`.
//   - Dropping the pool shuts it down gracefully. Close the channel by
//     dropping the Sender, so each worker's `recv()` returns Err once the
//     queue is empty and the worker exits its loop. Then `join` every
//     worker thread, so `drop` returns only after all queued jobs ran.
//
// Watch out:
//   - Hold the Mutex lock only while *receiving* a job, not while running
//     it, or the workers will take turns instead of running in parallel.
//     (Think about when a `MutexGuard` temporary is dropped.)
//   - If you get the shutdown order wrong, the tests can hang. Ctrl+C
//     stops them.
//
// Further reading:
//   - The Rust book builds exactly this in its final project:
//     https://doc.rust-lang.org/book/ch21-02-multithreaded.html
//     https://doc.rust-lang.org/book/ch21-03-graceful-shutdown-and-cleanup.html
//     Try it yourself first! Then compare.
//   - Fearless concurrency (ch. 16): https://doc.rust-lang.org/book/ch16-00-concurrency.html
//   - Channels: https://doc.rust-lang.org/std/sync/mpsc/index.html
//   - Trait objects, for `Box<dyn FnOnce()>`:
//     https://doc.rust-lang.org/book/ch18-02-trait-objects.html
//   - Atomics, used in the tests: https://doc.rust-lang.org/std/sync/atomic/index.html

// I AM NOT DONE

use std::sync::{Arc, Mutex, mpsc};
use std::thread;

/// A job is any closure that can be sent to another thread and run once.
type Job = Box<dyn FnOnce() + Send + 'static>;

struct ThreadPool {
    // TODO: what does the pool need to keep hold of?
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        todo!()
    }

    fn size(&self) -> usize {
        todo!()
    }

    fn execute<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        todo!()
    }
}

// TODO: impl Drop for ThreadPool { ... }

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
        assert!(
            start.elapsed() < Duration::from_millis(900),
            "took {:?}",
            start.elapsed()
        );
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

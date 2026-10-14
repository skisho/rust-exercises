// threads1.rs - solution
//
// `JoinHandle::join` blocks until the thread finishes and returns
// `Result<T, _>`: `Err` if the thread panicked.

use std::thread;

fn squares_in_threads() -> Vec<u64> {
    let mut handles = Vec::new();
    for i in 0..10u64 {
        let handle = thread::spawn(move || i * i);
        handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
        results.push(handle.join().unwrap());
    }
    results
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_results() {
        assert_eq!(squares_in_threads(), [0, 1, 4, 9, 16, 25, 36, 49, 64, 81]);
    }
}

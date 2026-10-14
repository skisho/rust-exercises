// threads2.rs - solution
//
// Each thread gets its own `Arc` clone pointing at the same `Mutex`.
// `lock()` returns a guard that derefs to the data and unlocks on drop.

use std::sync::{Arc, Mutex};
use std::thread;

fn count_with_threads() -> u32 {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            *counter.lock().unwrap() += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    *counter.lock().unwrap()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_to_ten() {
        assert_eq!(count_with_threads(), 10);
    }
}

// threads2.rs
//
// Ten threads each add 1 to a shared counter. Make it compile using
// `Arc` (shared ownership across threads) and `Mutex` (exclusive access).

// I AM NOT DONE

use std::sync::{Arc, Mutex};
use std::thread;

fn count_with_threads() -> u32 {
    let counter = 0;
    let mut handles = Vec::new();

    for _ in 0..10 {
        let handle = thread::spawn(move || {
            counter += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    counter
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

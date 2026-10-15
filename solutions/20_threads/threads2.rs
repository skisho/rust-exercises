// threads2.rs - solution
//
// Each thread gets its own clone of the `Arc` (a new pointer to the same
// Mutex). `lock()` blocks until no other thread holds the lock; the guard
// it returns unlocks automatically when it goes out of scope.
//
// Locking once per station, rather than once per ballot, keeps the
// threads from waiting on each other more than necessary.
//
// At the end every thread has finished, so ours is the only `Arc` left.
// `Arc::try_unwrap` takes the Mutex back out, and `into_inner` takes the
// HashMap out of the Mutex.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

fn count_votes(stations: Vec<Vec<&'static str>>) -> HashMap<&'static str, u32> {
    let tally = Arc::new(Mutex::new(HashMap::new()));
    let mut handles = Vec::new();

    for ballots in stations {
        let tally = Arc::clone(&tally);
        let handle = thread::spawn(move || {
            let mut tally = tally.lock().unwrap();
            for candidate in ballots {
                *tally.entry(candidate).or_insert(0) += 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Arc::try_unwrap(tally).unwrap().into_inner().unwrap()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_all_stations() {
        let stations = vec![
            vec!["crab", "crab", "octopus"],
            vec!["octopus", "crab"],
            vec!["squid", "crab", "octopus", "crab"],
        ];
        let result = count_votes(stations);
        assert_eq!(result["crab"], 5);
        assert_eq!(result["octopus"], 3);
        assert_eq!(result["squid"], 1);
        assert_eq!(result.len(), 3);
    }
}

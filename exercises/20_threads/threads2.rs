// threads2.rs
//
// An election: each polling station counts its own ballots on a separate
// thread, adding its counts into one shared tally.
//
// It doesn't compile, because every thread wants to own and modify the
// same HashMap. Share it with `Arc` (shared ownership across threads)
// and protect it with `Mutex` (one thread at a time).

// I AM NOT DONE

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

fn count_votes(stations: Vec<Vec<&'static str>>) -> HashMap<&'static str, u32> {
    let tally = HashMap::new();
    let mut handles = Vec::new();

    for ballots in stations {
        let handle = thread::spawn(move || {
            for candidate in ballots {
                *tally.entry(candidate).or_insert(0) += 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    tally
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

// threads3.rs
//
// Instead of sharing memory, threads can communicate by sending messages
// through a channel. Three workers each send three numbers; the main
// thread collects them all.
//
// Make it compile. Then make sure it *finishes*: `rx.iter()` keeps
// waiting until every sender has been dropped. (If the test seems to
// hang, press Ctrl+C and think about which senders are still alive.)

// I AM NOT DONE

use std::sync::mpsc;
use std::thread;

fn collect_from_workers() -> Vec<u32> {
    let (tx, rx) = mpsc::channel();

    for id in 0..3 {
        thread::spawn(move || {
            for i in 0..3 {
                tx.send(id * 10 + i).unwrap();
            }
        });
    }

    let mut results: Vec<u32> = rx.iter().collect();
    results.sort();
    results
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_everything() {
        assert_eq!(collect_from_workers(), [0, 1, 2, 10, 11, 12, 20, 21, 22]);
    }
}

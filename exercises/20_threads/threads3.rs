// threads3.rs
//
// A two-stage pipeline connected by channels:
//
//     producer --(raw numbers)--> squarer --(squares)--> main thread
//
// The producer sends 1 to 5, the squarer receives each number and sends
// on its square, and the main thread collects the results.
//
// 1. Make it compile. The closures need to *own* the channel ends they
//    use.
// 2. Make sure it *finishes*. A `for` loop over a receiver ends only when
//    every sender for that channel has been dropped. If the test hangs,
//    press Ctrl+C and think about who is still holding a sender.

// I AM NOT DONE

use std::sync::mpsc;
use std::thread;

fn squares_pipeline() -> Vec<u64> {
    let (raw_tx, raw_rx) = mpsc::channel();
    let (squared_tx, squared_rx) = mpsc::channel();

    thread::spawn(|| {
        for n in 1..=5u64 {
            raw_tx.send(n).unwrap();
        }
    });

    thread::spawn(|| {
        for n in raw_rx {
            squared_tx.send(n * n).unwrap();
        }
    });

    // The main thread keeps a sender too, to report a final marker.
    let report_tx = squared_tx.clone();
    report_tx.send(0).unwrap();

    let mut results: Vec<u64> = squared_rx.iter().collect();
    results.sort();
    results
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn squares_one_to_five() {
        assert_eq!(squares_pipeline(), [0, 1, 4, 9, 16, 25]);
    }
}

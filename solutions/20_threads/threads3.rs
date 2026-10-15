// threads3.rs - solution
//
// `move` makes each closure take ownership of the channel ends it uses.
//
// The hang came from `report_tx` (and the order of the clone): the main
// thread still held a sender, so the channel never closed and `iter()`
// waited forever. The clone must also happen *before* `squared_tx` moves
// into the squarer. Dropping `report_tx` once it's done closes our side.
// The producer's and squarer's senders are dropped when their threads
// end, so the chain shuts down in order: producer, squarer, main.

use std::sync::mpsc;
use std::thread;

fn squares_pipeline() -> Vec<u64> {
    let (raw_tx, raw_rx) = mpsc::channel();
    let (squared_tx, squared_rx) = mpsc::channel();

    // The main thread keeps a sender too, to report a final marker.
    let report_tx = squared_tx.clone();

    thread::spawn(move || {
        for n in 1..=5u64 {
            raw_tx.send(n).unwrap();
        }
    });

    thread::spawn(move || {
        for n in raw_rx {
            squared_tx.send(n * n).unwrap();
        }
    });

    report_tx.send(0).unwrap();
    drop(report_tx);

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

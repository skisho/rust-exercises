// threads3.rs - solution
//
// Each worker needs its own sender, so clone `tx` before moving it into
// the closure. The receiver's iterator only ends once *all* senders are
// gone, including the original `tx`, so drop it once the workers have
// their clones.

use std::sync::mpsc;
use std::thread;

fn collect_from_workers() -> Vec<u32> {
    let (tx, rx) = mpsc::channel();

    for id in 0..3 {
        let tx = tx.clone();
        thread::spawn(move || {
            for i in 0..3 {
                tx.send(id * 10 + i).unwrap();
            }
        });
    }
    drop(tx);

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

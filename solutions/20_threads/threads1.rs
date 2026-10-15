// threads1.rs - solution
//
// `thread::scope(|s| ...)` waits for every thread spawned with `s.spawn`
// before it returns. Since the threads can't outlive the scope, they're
// allowed to borrow anything that lives longer than it, like `numbers`.
// `join` returns the closure's result, just as with `thread::spawn`.
//
// Before scoped threads existed, you'd have needed `Arc` or copies of the
// data to do this.

use std::thread;

fn parallel_sum(numbers: &[u64], chunk_size: usize) -> u64 {
    thread::scope(|s| {
        let mut handles = Vec::new();
        for chunk in numbers.chunks(chunk_size) {
            handles.push(s.spawn(|| chunk.iter().sum::<u64>()));
        }

        let mut total = 0;
        for handle in handles {
            total += handle.join().unwrap();
        }
        total
    })
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_everything() {
        let numbers: Vec<u64> = (1..=1000).collect();
        assert_eq!(parallel_sum(&numbers, 100), 500_500);
    }

    #[test]
    fn uneven_chunks() {
        assert_eq!(parallel_sum(&[1, 2, 3, 4, 5], 2), 15);
    }

    #[test]
    fn nothing_to_sum() {
        assert_eq!(parallel_sum(&[], 10), 0);
    }
}

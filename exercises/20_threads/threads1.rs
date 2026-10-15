// threads1.rs
//
// `parallel_sum` splits a slice into chunks and sums each chunk on its own
// thread. It doesn't compile: `thread::spawn` requires its closure to own
// everything it uses (`'static`), because the thread might outlive the
// function. But here the threads borrow `numbers`.
//
// We *know* every thread finishes before `parallel_sum` returns. Tell the
// compiler so by using `thread::scope`, which lets threads borrow local
// data. Don't copy the numbers into new Vecs.

// I AM NOT DONE

use std::thread;

fn parallel_sum(numbers: &[u64], chunk_size: usize) -> u64 {
    let mut handles = Vec::new();
    for chunk in numbers.chunks(chunk_size) {
        handles.push(thread::spawn(|| chunk.iter().sum::<u64>()));
    }

    let mut total = 0;
    for handle in handles {
        total += handle.join().unwrap();
    }
    total
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

// iterators4.rs
//
// Any type can become an iterator by implementing the `Iterator` trait:
// declare the `Item` type and write `next`. Then every adapter (`take`,
// `filter`, `sum`, ...) works on it for free.
//
// Implement an infinite Fibonacci iterator: 0, 1, 1, 2, 3, 5, 8, ...

// I AM NOT DONE

struct Fibonacci {
    current: u64,
    next: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci {
            current: 0,
            next: 1,
        }
    }
}

impl Iterator for Fibonacci {
    // TODO: declare the item type and implement `next`.
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_ten() {
        let fibs: Vec<u64> = Fibonacci::new().take(10).collect();
        assert_eq!(fibs, [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }

    #[test]
    fn works_with_adapters() {
        // The sum of the even Fibonacci numbers below 100.
        let sum: u64 = Fibonacci::new()
            .take_while(|&n| n < 100)
            .filter(|n| n % 2 == 0)
            .sum();
        assert_eq!(sum, 0 + 2 + 8 + 34);
    }
}

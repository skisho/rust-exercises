// iterators4.rs - solution
//
// `next` returns `Some(item)` to keep going, or `None` to stop. This one
// never stops, which is fine: iterators are lazy, so nothing runs until
// something like `take` or `collect` asks for items.

struct Fibonacci {
    current: u64,
    next: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { current: 0, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let value = self.current;
        self.current = self.next;
        self.next += value;
        Some(value)
    }
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
        let sum: u64 = Fibonacci::new()
            .take_while(|&n| n < 100)
            .filter(|n| n % 2 == 0)
            .sum();
        assert_eq!(sum, 0 + 2 + 8 + 34);
    }
}

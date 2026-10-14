// tests1.rs - solution
//
// `assert!(cond)` panics (failing the test) when `cond` is false.

fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        assert!(is_even(4));
    }

    #[test]
    fn is_false_when_odd() {
        assert!(!is_even(5));
    }
}

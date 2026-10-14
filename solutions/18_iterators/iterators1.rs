// iterators1.rs - solution
//
// `iter()` yields `&i32`, so the closures receive references. Arithmetic
// operators work on `&i32` directly, which is why `n * n` compiles.

fn sum_of_even_squares(numbers: &[i32]) -> i32 {
    numbers.iter().filter(|n| *n % 2 == 0).map(|n| n * n).sum()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mixed() {
        assert_eq!(sum_of_even_squares(&[1, 2, 3, 4, 5, 6]), 56);
    }

    #[test]
    fn no_evens() {
        assert_eq!(sum_of_even_squares(&[1, 3, 5]), 0);
    }

    #[test]
    fn empty() {
        assert_eq!(sum_of_even_squares(&[]), 0);
    }
}

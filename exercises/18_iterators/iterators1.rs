// iterators1.rs
//
// Iterators let you describe *what* to compute rather than writing loops.
// Implement `sum_of_even_squares` as a single iterator chain: no `for`,
// no `mut`.

// I AM NOT DONE

fn sum_of_even_squares(numbers: &[i32]) -> i32 {
    // TODO: keep the even numbers, square each one, and add them up.
    // Useful adapters: `iter`, `filter`, `map`, `sum`.
    todo!()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mixed() {
        // 2*2 + 4*4 + 6*6 = 56
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

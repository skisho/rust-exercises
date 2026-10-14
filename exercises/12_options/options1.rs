// options1.rs
//
// `Option<T>` is how Rust represents a value that might be missing.
// Implement `first_even`, then fix the test so it compiles.

// I AM NOT DONE

fn first_even(numbers: &[i32]) -> Option<i32> {
    // TODO: return the first even number, or None if there isn't one.
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_even() {
        assert_eq!(first_even(&[1, 3, 4, 6]), Some(4));
    }

    #[test]
    fn no_even() {
        assert_eq!(first_even(&[1, 3, 5]), None);
    }

    #[test]
    fn use_the_value() {
        // TODO: fix this line. You can't add an Option<i32> to an i32!
        let doubled: i32 = first_even(&[7, 10]) * 2;
        assert_eq!(doubled, 20);
    }
}

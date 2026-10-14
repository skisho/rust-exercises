// tests1.rs
//
// Tests are functions marked with `#[test]`. Fill in the assertions so
// they check `is_even` properly (and pass!).

// I AM NOT DONE

fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        assert!(???);
    }

    #[test]
    fn is_false_when_odd() {
        assert!(???);
    }
}

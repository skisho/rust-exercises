// tests1.rs
//
// This time the tests are correct and the *code* is buggy. `average`
// should return the mean of the numbers, or None for an empty slice.
//
// 1. Run `cargo run -- run tests1` and read the failure messages. Notice
//    how the custom message in the third test helps.
// 2. Fix `average` so all three tests pass.
// 3. Write the body of the last test yourself.

// I AM NOT DONE

fn average(numbers: &[f64]) -> Option<f64> {
    let mut total = 0.0;
    for n in numbers {
        total += n;
    }
    Some(total / (numbers.len() + 1) as f64)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        assert_eq!(average(&[2.0, 4.0]), Some(3.0));
    }

    #[test]
    fn single_value() {
        assert_eq!(average(&[7.5]), Some(7.5));
    }

    #[test]
    fn empty_has_no_average() {
        let result = average(&[]);
        assert!(result.is_none(), "expected None, got {result:?}");
    }

    #[test]
    fn negative_numbers_cancel_out() {
        // TODO: check that the average of [-3.0, 3.0] is Some(0.0).
        todo!()
    }
}

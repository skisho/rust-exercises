// tests1.rs - solution
//
// The off-by-one (`len() + 1`) made every average too small, and dividing
// by zero isn't an error for floats: it gives NaN, which is why the empty
// case needs its own check.
//
// `assert!(cond, "format", args...)` and `assert_eq!(a, b, "...")` accept
// a message that's printed when they fail. Good messages make failures
// much quicker to understand.

fn average(numbers: &[f64]) -> Option<f64> {
    if numbers.is_empty() {
        return None;
    }
    let mut total = 0.0;
    for n in numbers {
        total += n;
    }
    Some(total / numbers.len() as f64)
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
        assert_eq!(average(&[-3.0, 3.0]), Some(0.0));
    }
}

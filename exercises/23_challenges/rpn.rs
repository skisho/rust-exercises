// rpn.rs (challenge)
//
// CHALLENGES take longer than the regular exercises. Expect to read some
// documentation, and to write code from scratch rather than fill in a
// single gap. Take your time: `cargo run -- run rpn` shows the full test
// output while you work.
//
// ---------------------------------------------------------------------
//
// Build a Reverse Polish Notation (RPN) calculator. In RPN the operator
// comes *after* its operands, so no brackets are ever needed:
//
//     "3 4 +"               => 7
//     "5 1 2 + 4 * + 3 -"   => 5 + ((1 + 2) * 4) - 3 = 14
//
// Algorithm: read tokens left to right. Push numbers onto a stack. For an
// operator, pop two values, apply it, and push the result. At the end
// exactly one value should remain.
//
// Requirements:
//   1. `eval_rpn` supports `+ - * /` on i64 values (integer division).
//      Negative numbers like "-3" are numbers, not the `-` operator.
//   2. Each failure returns the matching `RpnError` variant (see tests).
//   3. Implement `Display` for `RpnError` with these messages:
//        InvalidToken(t)   -> "invalid token `t`"
//        StackUnderflow    -> "not enough values on the stack"
//        DivisionByZero    -> "division by zero"
//        TooManyValues(n)  -> "n values left on the stack, expected 1"
//        Empty             -> "empty expression"
//   4. Implement `std::error::Error` for `RpnError`, so it works with
//      `Box<dyn Error>` and `?`.
//
// Further reading:
//   - Reverse Polish notation: https://en.wikipedia.org/wiki/Reverse_Polish_notation
//   - Error handling (Rust book ch. 9): https://doc.rust-lang.org/book/ch09-00-error-handling.html
//   - The `Error` trait: https://doc.rust-lang.org/std/error/trait.Error.html
//   - Implementing Display: https://doc.rust-lang.org/rust-by-example/hello/print/print_display.html

// I AM NOT DONE

use std::fmt;

#[derive(Debug, PartialEq)]
enum RpnError {
    InvalidToken(String),
    StackUnderflow,
    DivisionByZero,
    TooManyValues(usize),
    Empty,
}

// TODO: impl fmt::Display for RpnError { ... }

// TODO: impl std::error::Error for RpnError {}

fn eval_rpn(input: &str) -> Result<i64, RpnError> {
    todo!()
}

fn main() {
    for expr in ["3 4 +", "5 1 2 + 4 * + 3 -", "1 0 /"] {
        match eval_rpn(expr) {
            Ok(value) => println!("{expr} = {value}"),
            Err(e) => println!("{expr}: error: {e}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        assert_eq!(eval_rpn("3 4 +"), Ok(7));
        assert_eq!(eval_rpn("10 4 -"), Ok(6));
        assert_eq!(eval_rpn("6 7 *"), Ok(42));
        assert_eq!(eval_rpn("7 2 /"), Ok(3));
    }

    #[test]
    fn single_number() {
        assert_eq!(eval_rpn("42"), Ok(42));
    }

    #[test]
    fn nested() {
        assert_eq!(eval_rpn("5 1 2 + 4 * + 3 -"), Ok(14));
    }

    #[test]
    fn negative_numbers() {
        assert_eq!(eval_rpn("-3 2 *"), Ok(-6));
        assert_eq!(eval_rpn("2 -3 -"), Ok(5));
    }

    #[test]
    fn extra_whitespace() {
        assert_eq!(eval_rpn("  1   2\t+ "), Ok(3));
    }

    #[test]
    fn errors() {
        assert_eq!(
            eval_rpn("2 x +"),
            Err(RpnError::InvalidToken("x".to_string()))
        );
        assert_eq!(eval_rpn("1 +"), Err(RpnError::StackUnderflow));
        assert_eq!(eval_rpn("1 0 /"), Err(RpnError::DivisionByZero));
        assert_eq!(eval_rpn("1 2 3 +"), Err(RpnError::TooManyValues(2)));
        assert_eq!(eval_rpn(""), Err(RpnError::Empty));
    }

    #[test]
    fn error_messages() {
        assert_eq!(
            RpnError::InvalidToken("x".into()).to_string(),
            "invalid token `x`"
        );
        assert_eq!(
            RpnError::StackUnderflow.to_string(),
            "not enough values on the stack"
        );
        assert_eq!(RpnError::DivisionByZero.to_string(), "division by zero");
        assert_eq!(
            RpnError::TooManyValues(3).to_string(),
            "3 values left on the stack, expected 1"
        );
        assert_eq!(RpnError::Empty.to_string(), "empty expression");
    }

    #[test]
    fn works_as_boxed_error() {
        fn run() -> Result<i64, Box<dyn std::error::Error>> {
            Ok(eval_rpn("1 0 /")?)
        }
        assert_eq!(run().unwrap_err().to_string(), "division by zero");
    }
}

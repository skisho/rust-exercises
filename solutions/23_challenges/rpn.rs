// rpn.rs (challenge) - solution
//
// Checking for an exact operator match *before* trying to parse a number
// is what makes "-3" a number and "-" an operator. Note the pop order:
// the first value popped is the *right-hand* operand.
//
// Since `Error` has default implementations for all its methods, an empty
// `impl Error for RpnError {}` is enough once `Debug` and `Display` exist.

use std::fmt;

#[derive(Debug, PartialEq)]
enum RpnError {
    InvalidToken(String),
    StackUnderflow,
    DivisionByZero,
    TooManyValues(usize),
    Empty,
}

impl fmt::Display for RpnError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RpnError::InvalidToken(token) => write!(f, "invalid token `{token}`"),
            RpnError::StackUnderflow => write!(f, "not enough values on the stack"),
            RpnError::DivisionByZero => write!(f, "division by zero"),
            RpnError::TooManyValues(n) => write!(f, "{n} values left on the stack, expected 1"),
            RpnError::Empty => write!(f, "empty expression"),
        }
    }
}

impl std::error::Error for RpnError {}

fn eval_rpn(input: &str) -> Result<i64, RpnError> {
    let mut stack: Vec<i64> = Vec::new();

    for token in input.split_whitespace() {
        match token {
            "+" | "-" | "*" | "/" => {
                let b = stack.pop().ok_or(RpnError::StackUnderflow)?;
                let a = stack.pop().ok_or(RpnError::StackUnderflow)?;
                let value = match token {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    _ => {
                        if b == 0 {
                            return Err(RpnError::DivisionByZero);
                        }
                        a / b
                    }
                };
                stack.push(value);
            }
            _ => {
                let n = token
                    .parse()
                    .map_err(|_| RpnError::InvalidToken(token.to_string()))?;
                stack.push(n);
            }
        }
    }

    match stack.len() {
        0 => Err(RpnError::Empty),
        1 => Ok(stack[0]),
        n => Err(RpnError::TooManyValues(n)),
    }
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
        assert_eq!(eval_rpn("2 x +"), Err(RpnError::InvalidToken("x".to_string())));
        assert_eq!(eval_rpn("1 +"), Err(RpnError::StackUnderflow));
        assert_eq!(eval_rpn("1 0 /"), Err(RpnError::DivisionByZero));
        assert_eq!(eval_rpn("1 2 3 +"), Err(RpnError::TooManyValues(2)));
        assert_eq!(eval_rpn(""), Err(RpnError::Empty));
    }

    #[test]
    fn error_messages() {
        assert_eq!(RpnError::InvalidToken("x".into()).to_string(), "invalid token `x`");
        assert_eq!(RpnError::StackUnderflow.to_string(), "not enough values on the stack");
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

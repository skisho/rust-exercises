// errors3.rs
//
// `parse_positive` can fail in two different ways: the text might not be
// a number, or the number might not be positive. Both should become a
// `ParsePosError`.
//
// 1. Implement `From<ParseIntError>` for `ParsePosError` so `?` can
//    convert the parse error automatically.
// 2. Finish `parse_positive`.

// I AM NOT DONE

use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
enum ParsePosError {
    NotANumber(ParseIntError),
    NotPositive(i64),
}

// TODO: impl From<ParseIntError> for ParsePosError { ... }

fn parse_positive(s: &str) -> Result<u64, ParsePosError> {
    let n: i64 = s.parse()?;
    // TODO: return the right error if n is zero or negative,
    // otherwise return it as a u64.
    Ok(n as u64)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid() {
        assert_eq!(parse_positive("42"), Ok(42));
    }

    #[test]
    fn not_a_number() {
        assert!(matches!(
            parse_positive("forty-two"),
            Err(ParsePosError::NotANumber(_))
        ));
    }

    #[test]
    fn not_positive() {
        assert_eq!(parse_positive("0"), Err(ParsePosError::NotPositive(0)));
        assert_eq!(parse_positive("-7"), Err(ParsePosError::NotPositive(-7)));
    }
}

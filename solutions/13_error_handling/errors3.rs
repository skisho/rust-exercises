// errors3.rs - solution
//
// When `?` sees an error of type A in a function returning Result<_, B>,
// it calls `B::from(a)`. Implementing `From` is how you teach it to
// convert between error types.

use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
enum ParsePosError {
    NotANumber(ParseIntError),
    NotPositive(i64),
}

impl From<ParseIntError> for ParsePosError {
    fn from(err: ParseIntError) -> Self {
        ParsePosError::NotANumber(err)
    }
}

fn parse_positive(s: &str) -> Result<u64, ParsePosError> {
    let n: i64 = s.parse()?;
    if n <= 0 {
        return Err(ParsePosError::NotPositive(n));
    }
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

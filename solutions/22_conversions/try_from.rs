// try_from.rs - solution
//
// `u8::try_from(i16)` already does the range check for us; `map_err`
// swaps its error type for ours so `?` can be used.

use std::convert::TryFrom;

#[derive(Debug, PartialEq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug, PartialEq)]
enum IntoColorError {
    OutOfRange,
}

impl TryFrom<(i16, i16, i16)> for Color {
    type Error = IntoColorError;

    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        let component = |n: i16| u8::try_from(n).map_err(|_| IntoColorError::OutOfRange);
        Ok(Color {
            red: component(tuple.0)?,
            green: component(tuple.1)?,
            blue: component(tuple.2)?,
        })
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid() {
        assert_eq!(
            Color::try_from((183, 65, 14)),
            Ok(Color { red: 183, green: 65, blue: 14 })
        );
    }

    #[test]
    fn try_into_works_for_free() {
        let c: Result<Color, _> = (0, 0, 255).try_into();
        assert!(c.is_ok());
    }

    #[test]
    fn out_of_range() {
        assert_eq!(Color::try_from((256, 0, 0)), Err(IntoColorError::OutOfRange));
        assert_eq!(Color::try_from((0, -1, 0)), Err(IntoColorError::OutOfRange));
        assert_eq!(Color::try_from((0, 0, 300)), Err(IntoColorError::OutOfRange));
    }
}

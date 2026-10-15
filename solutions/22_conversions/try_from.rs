// try_from.rs - solution
//
// `TryFrom` has an associated `Error` type, so each conversion picks its
// own error. A `match` with a catch-all `_` arm covers "everything else".
// For names, lowercase the input once with `to_ascii_lowercase`, then
// match on `.as_str()`. As with `From`/`Into`, implementing `TryFrom`
// gives you `try_into()` for free.

use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Debug, PartialEq)]
enum WeekdayError {
    OutOfRange(u8),
    UnknownName(String),
}

impl TryFrom<u8> for Weekday {
    type Error = WeekdayError;

    fn try_from(n: u8) -> Result<Self, Self::Error> {
        match n {
            1 => Ok(Weekday::Monday),
            2 => Ok(Weekday::Tuesday),
            3 => Ok(Weekday::Wednesday),
            4 => Ok(Weekday::Thursday),
            5 => Ok(Weekday::Friday),
            6 => Ok(Weekday::Saturday),
            7 => Ok(Weekday::Sunday),
            _ => Err(WeekdayError::OutOfRange(n)),
        }
    }
}

impl TryFrom<&str> for Weekday {
    type Error = WeekdayError;

    fn try_from(name: &str) -> Result<Self, Self::Error> {
        match name.to_ascii_lowercase().as_str() {
            "mon" => Ok(Weekday::Monday),
            "tue" => Ok(Weekday::Tuesday),
            "wed" => Ok(Weekday::Wednesday),
            "thu" => Ok(Weekday::Thursday),
            "fri" => Ok(Weekday::Friday),
            "sat" => Ok(Weekday::Saturday),
            "sun" => Ok(Weekday::Sunday),
            _ => Err(WeekdayError::UnknownName(name.to_string())),
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_numbers() {
        assert_eq!(Weekday::try_from(1), Ok(Weekday::Monday));
        assert_eq!(Weekday::try_from(7), Ok(Weekday::Sunday));
        assert_eq!(Weekday::try_from(0), Err(WeekdayError::OutOfRange(0)));
        assert_eq!(Weekday::try_from(8), Err(WeekdayError::OutOfRange(8)));
    }

    #[test]
    fn from_names() {
        assert_eq!(Weekday::try_from("wed"), Ok(Weekday::Wednesday));
        assert_eq!(Weekday::try_from("FRI"), Ok(Weekday::Friday));
        assert_eq!(
            Weekday::try_from("Funday"),
            Err(WeekdayError::UnknownName("Funday".to_string()))
        );
    }

    #[test]
    fn try_into_works_for_free() {
        let day: Result<Weekday, _> = 6u8.try_into();
        assert_eq!(day, Ok(Weekday::Saturday));
        let day: Result<Weekday, _> = "sun".try_into();
        assert_eq!(day, Ok(Weekday::Sunday));
    }
}

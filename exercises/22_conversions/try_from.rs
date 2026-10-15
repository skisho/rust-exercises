// try_from.rs
//
// `TryFrom` is for conversions that can fail. Implement two ways to get a
// `Weekday`:
//   - `TryFrom<u8>`: 1 is Monday ... 7 is Sunday. Anything else is
//     `WeekdayError::OutOfRange(n)`.
//   - `TryFrom<&str>`: the first three letters of the English name, in
//     any case ("mon", "TUE", "Wed"...). Anything else is
//     `WeekdayError::UnknownName(the input)`.

// I AM NOT DONE

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
        todo!()
    }
}

// TODO: impl TryFrom<&str> for Weekday { ... }

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

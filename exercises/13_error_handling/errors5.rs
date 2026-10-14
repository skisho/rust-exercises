// errors5.rs
//
// Look up a user's age from a map of "name" -> "age as text" and return
// it as a number. Each step can fail differently:
//   - the name isn't in the map      -> LookupError::Missing(name)
//   - the text isn't a valid number  -> LookupError::BadAge(text)
//
// Implement `age_of` WITHOUT `match` or `if let`. Use the combinators
// `ok_or_else` (Option -> Result) and `map_err` (change the error), plus `?`.

// I AM NOT DONE

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum LookupError {
    Missing(String),
    BadAge(String),
}

fn age_of(people: &HashMap<&str, &str>, name: &str) -> Result<u32, LookupError> {
    todo!()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    fn people() -> HashMap<&'static str, &'static str> {
        HashMap::from([("ada", "36"), ("alan", "forty-one")])
    }

    #[test]
    fn found() {
        assert_eq!(age_of(&people(), "ada"), Ok(36));
    }

    #[test]
    fn missing() {
        assert_eq!(
            age_of(&people(), "grace"),
            Err(LookupError::Missing("grace".to_string()))
        );
    }

    #[test]
    fn bad_age() {
        assert_eq!(
            age_of(&people(), "alan"),
            Err(LookupError::BadAge("forty-one".to_string()))
        );
    }
}

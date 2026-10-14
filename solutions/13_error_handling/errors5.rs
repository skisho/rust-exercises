// errors5.rs - solution
//
// `ok_or_else` turns `Option<T>` into `Result<T, E>`, building the error
// lazily. `map_err` rewrites only the error, leaving `Ok` untouched.
// Chaining these with `?` keeps fallible code flat and readable.

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum LookupError {
    Missing(String),
    BadAge(String),
}

fn age_of(people: &HashMap<&str, &str>, name: &str) -> Result<u32, LookupError> {
    let text = people
        .get(name)
        .ok_or_else(|| LookupError::Missing(name.to_string()))?;
    text.parse::<u32>()
        .map_err(|_| LookupError::BadAge(text.to_string()))
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

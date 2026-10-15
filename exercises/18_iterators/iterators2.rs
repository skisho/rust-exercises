// iterators2.rs
//
// `collect` can build many different types. What it builds depends on the
// type you ask for. Implement each function as a single iterator chain
// ending in `collect` (plus a little extra where noted).

// I AM NOT DONE

use std::collections::HashMap;
use std::num::ParseIntError;

// "one two three" -> "three two one"
// Collect the words into a Vec, then `join` them.
fn reverse_words(sentence: &str) -> String {
    todo!()
}

// ["apple", "fig"] -> {"apple": 5, "fig": 3}
// An iterator of (key, value) tuples collects into a HashMap.
fn lengths<'a>(words: &[&'a str]) -> HashMap<&'a str, usize> {
    todo!()
}

// ["1", "2", "3"] -> Ok([1, 2, 3]), but ["1", "x"] -> Err(...)
// An iterator of Results can collect into a Result of a Vec. It stops at
// the first error.
fn parse_all(items: &[&str]) -> Result<Vec<i32>, ParseIntError> {
    todo!()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reversing() {
        assert_eq!(reverse_words("one two three"), "three two one");
        assert_eq!(reverse_words("solo"), "solo");
    }

    #[test]
    fn word_lengths() {
        let map = lengths(&["apple", "fig", "kiwi"]);
        assert_eq!(map.len(), 3);
        assert_eq!(map["apple"], 5);
        assert_eq!(map["fig"], 3);
    }

    #[test]
    fn all_good() {
        assert_eq!(parse_all(&["1", "-2", "30"]), Ok(vec![1, -2, 30]));
    }

    #[test]
    fn one_bad() {
        let err = parse_all(&["1", "x", "3"]).unwrap_err();
        assert_eq!(err.to_string(), "invalid digit found in string");
    }
}

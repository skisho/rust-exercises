// iterators2.rs - solution
//
// `collect` is driven by the `FromIterator` trait. The same chain can
// produce a Vec, a String, a HashMap, a HashSet... whatever the type
// annotation (or return type) asks for.
//
// Collecting `Result<T, E>` items into `Result<Vec<T>, E>` is especially
// handy: it gives you every value, or the first error.

use std::collections::HashMap;
use std::num::ParseIntError;

fn reverse_words(sentence: &str) -> String {
    sentence
        .split_whitespace()
        .rev()
        .collect::<Vec<_>>()
        .join(" ")
}

fn lengths<'a>(words: &[&'a str]) -> HashMap<&'a str, usize> {
    words.iter().map(|&w| (w, w.len())).collect()
}

fn parse_all(items: &[&str]) -> Result<Vec<i32>, ParseIntError> {
    items.iter().map(|s| s.parse::<i32>()).collect()
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

// hashmaps2.rs - solution
//
// `entry(key).or_insert(default)` returns a `&mut` to the value, inserting
// the default first if the key is missing. Perfect for counting.

use std::collections::HashMap;

fn word_count(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word.to_lowercase()).or_insert(0) += 1;
    }
    counts
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_words() {
        let counts = word_count("the cat and The hat and the bat");
        assert_eq!(counts["the"], 3);
        assert_eq!(counts["and"], 2);
        assert_eq!(counts["cat"], 1);
        assert_eq!(counts.len(), 5);
    }

    #[test]
    fn empty_text() {
        assert!(word_count("").is_empty());
    }
}

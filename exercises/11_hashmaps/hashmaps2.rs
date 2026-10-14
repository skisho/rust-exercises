// hashmaps2.rs
//
// Count how many times each word appears in a piece of text.
// Words are separated by whitespace and should be lowercased first.

// I AM NOT DONE

use std::collections::HashMap;

fn word_count(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        // TODO: increase the count for the lowercased word.
        // Look at the `entry` method and `or_insert`.
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

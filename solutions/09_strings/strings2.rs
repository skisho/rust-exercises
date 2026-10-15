// strings2.rs - solution
//
// - `split_whitespace` handles any amount of spacing. `chars().next()`
//   gets the first character safely, even for multi-byte characters.
//   `to_uppercase` returns an iterator, because some characters uppercase
//   to more than one (ß -> SS).
// - `is_palindrome` normalises the text into a Vec<char>, then compares it
//   with its reverse.
// - `"*".repeat(n)` builds the mask. Note `word.chars().count()`, not
//   `word.len()`, so the mask matches the visible length.

fn initials(full_name: &str) -> String {
    let mut result = String::new();
    for word in full_name.split_whitespace() {
        if let Some(first) = word.chars().next() {
            result.extend(first.to_uppercase());
            result.push('.');
        }
    }
    result
}

fn is_palindrome(text: &str) -> bool {
    let cleaned: Vec<char> = text
        .chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect();
    cleaned.iter().eq(cleaned.iter().rev())
}

fn censor(text: &str, word: &str) -> String {
    text.replace(word, &"*".repeat(word.chars().count()))
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_initials() {
        assert_eq!(initials("Grace Brewster Hopper"), "G.B.H.");
        assert_eq!(initials("  ada   lovelace "), "A.L.");
        assert_eq!(initials(""), "");
    }

    #[test]
    fn palindromes() {
        assert!(is_palindrome("racecar"));
        assert!(is_palindrome("Never odd or even"));
        assert!(is_palindrome("A man, a plan, a canal: Panama!"));
        assert!(!is_palindrome("Rustacean"));
    }

    #[test]
    fn censoring() {
        assert_eq!(censor("what the heck", "heck"), "what the ****");
        assert_eq!(censor("bug bug bug", "bug"), "*** *** ***");
        assert_eq!(censor("all clean", "dirt"), "all clean");
    }
}

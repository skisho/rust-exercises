// strings2.rs
//
// Implement each function. The docs for `str` and `char` are your
// friends here, especially `chars`, `split_whitespace`, `to_uppercase`,
// `is_alphanumeric` and `repeat`.

// I AM NOT DONE

// "Grace Brewster Hopper" -> "G.B.H."
fn initials(full_name: &str) -> String {
    todo!()
}

// Ignore case and anything that isn't a letter or digit:
// "Never odd or even" and "A man, a plan, a canal: Panama!" are palindromes.
fn is_palindrome(text: &str) -> bool {
    todo!()
}

// Replace every occurrence of `word` with as many '*' as it has letters.
fn censor(text: &str, word: &str) -> String {
    todo!()
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

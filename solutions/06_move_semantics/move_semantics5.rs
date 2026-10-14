// move_semantics5.rs - solution
//
// Returning `&str` borrows part of the input instead of copying it.
// Taking `&str` rather than `&String` accepts both, since `&String`
// coerces to `&str` automatically.

fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None => s,
    }
    // Or: s.split(' ').next().unwrap_or("")
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_string() {
        let sentence = String::from("hello wonderful world");
        assert_eq!(first_word(&sentence), "hello");
    }

    #[test]
    fn from_literal() {
        assert_eq!(first_word("single"), "single");
        assert_eq!(first_word(""), "");
    }
}

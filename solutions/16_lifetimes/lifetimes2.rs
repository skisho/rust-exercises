// lifetimes2.rs - solution
//
// `Words<'a>` means "a Words can't outlive the text it points into".
// The key is `next_word(&mut self) -> Option<&'a str>`. Without the `'a`,
// elision would tie the result to the `&mut self` borrow, so you couldn't
// call `next_word` again while holding the first word, and the words
// couldn't outlive the struct.

struct Words<'a> {
    rest: &'a str,
}

impl<'a> Words<'a> {
    fn new(text: &'a str) -> Words<'a> {
        Words { rest: text }
    }

    fn next_word(&mut self) -> Option<&'a str> {
        let trimmed = self.rest.trim_start();
        if trimmed.is_empty() {
            self.rest = trimmed;
            return None;
        }
        let end = trimmed.find(' ').unwrap_or(trimmed.len());
        let (word, rest) = trimmed.split_at(end);
        self.rest = rest;
        Some(word)
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_by_one() {
        let mut words = Words::new("  the quick  brown fox ");
        assert_eq!(words.next_word(), Some("the"));
        assert_eq!(words.next_word(), Some("quick"));
        assert_eq!(words.next_word(), Some("brown"));
        assert_eq!(words.next_word(), Some("fox"));
        assert_eq!(words.next_word(), None);
    }

    #[test]
    fn words_outlive_the_struct() {
        let text = String::from("hello world");
        let (first, second) = {
            let mut words = Words::new(&text);
            let first = words.next_word();
            let second = words.next_word();
            (first, second)
        };
        assert_eq!(first, Some("hello"));
        assert_eq!(second, Some("world"));
    }
}

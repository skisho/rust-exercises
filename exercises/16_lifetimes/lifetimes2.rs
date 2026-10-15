// lifetimes2.rs
//
// `Words` hands out the words of some text one at a time without copying
// them: each word is a slice of the original text. A struct that holds a
// reference needs a lifetime parameter.
//
// 1. Add the lifetime annotations `Words` needs.
// 2. Implement `next_word`. The words it returns borrow from the *text*,
//    not from the `Words` struct, so they stay valid after `Words` is
//    gone. Make sure the signature says so.

// I AM NOT DONE

struct Words {
    rest: &str,
}

impl Words {
    fn new(text: &str) -> Words {
        Words { rest: text }
    }

    fn next_word(&mut self) -> Option<&str> {
        // TODO: skip leading spaces, then split off everything up to the
        // next space (or the end). Return None when nothing is left.
        // `trim_start`, `find` and `split_at` are useful here.
        todo!()
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

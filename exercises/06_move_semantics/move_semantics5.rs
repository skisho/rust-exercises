// move_semantics5.rs
//
// `first_word` should return the first word of a sentence *without*
// allocating a new String: return a slice that borrows from the input.
// Fix the signature and implement it.

// I AM NOT DONE

fn first_word(s: &String) -> String {
    todo!()
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
        // A more flexible signature also accepts string literals.
        assert_eq!(first_word("single"), "single");
        assert_eq!(first_word(""), "");
    }
}

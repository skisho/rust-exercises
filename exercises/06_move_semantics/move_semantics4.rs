// move_semantics4.rs
//
// `add_plurals` should append the plural of every word that's already in
// the list: ["cat", "dog"] becomes ["cat", "dog", "cats", "dogs"].
//
// It doesn't compile: the `for` loop borrows `words` to read it, and
// pushing needs to borrow it mutably at the same time. Rust forbids that,
// because a push may reallocate the Vec and leave the loop reading freed
// memory.
//
// Restructure the function so the reading and the writing don't overlap.

// I AM NOT DONE

fn add_plurals(words: &mut Vec<String>) {
    for word in words.iter() {
        words.push(format!("{word}s"));
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pluralise() {
        let mut words = vec![String::from("cat"), String::from("dog")];
        add_plurals(&mut words);
        assert_eq!(words, ["cat", "dog", "cats", "dogs"]);
    }

    #[test]
    fn empty() {
        let mut words: Vec<String> = Vec::new();
        add_plurals(&mut words);
        assert!(words.is_empty());
    }
}

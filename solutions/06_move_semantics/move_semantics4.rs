// move_semantics4.rs - solution
//
// First finish reading: build the plurals in a separate Vec while `words`
// is only borrowed immutably. That borrow ends with the loop, so then
// `words` can be borrowed mutably to `extend` it.
//
// (Even in languages that allow it, modifying a list while iterating over
// it is a classic bug. Here this one would also loop forever!)

fn add_plurals(words: &mut Vec<String>) {
    let mut plurals = Vec::new();
    for word in words.iter() {
        plurals.push(format!("{word}s"));
    }
    words.extend(plurals);
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

// hashmaps1.rs
//
// A tiny phone book mapping names to numbers. Implement:
//   - `add`: store a number, replacing any old number for that name
//   - `lookup`: the number for a name, or None
//   - `rename`: move an entry to a new name. Return false (and change
//     nothing) if the old name isn't there or the new name is taken.
//
// Useful methods: `insert`, `get`, `remove`, `contains_key`.

// I AM NOT DONE

use std::collections::HashMap;

struct PhoneBook {
    entries: HashMap<String, String>,
}

impl PhoneBook {
    fn new() -> Self {
        PhoneBook {
            entries: HashMap::new(),
        }
    }

    fn add(&mut self, name: &str, number: &str) {
        todo!()
    }

    fn lookup(&self, name: &str) -> Option<&str> {
        todo!()
    }

    fn rename(&mut self, old: &str, new: &str) -> bool {
        todo!()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_and_lookup() {
        let mut book = PhoneBook::new();
        book.add("Ada", "555-0101");
        book.add("Alan", "555-0199");
        assert_eq!(book.lookup("Ada"), Some("555-0101"));
        assert_eq!(book.lookup("Grace"), None);
    }

    #[test]
    fn add_replaces() {
        let mut book = PhoneBook::new();
        book.add("Ada", "555-0101");
        book.add("Ada", "555-0202");
        assert_eq!(book.lookup("Ada"), Some("555-0202"));
        assert_eq!(book.entries.len(), 1);
    }

    #[test]
    fn renaming() {
        let mut book = PhoneBook::new();
        book.add("Ada", "555-0101");
        book.add("Alan", "555-0199");

        assert!(book.rename("Ada", "Ada Lovelace"));
        assert_eq!(book.lookup("Ada"), None);
        assert_eq!(book.lookup("Ada Lovelace"), Some("555-0101"));

        assert!(!book.rename("Nobody", "Someone"));
        assert!(!book.rename("Alan", "Ada Lovelace"));
        assert_eq!(book.lookup("Alan"), Some("555-0199"));
        assert_eq!(book.lookup("Ada Lovelace"), Some("555-0101"));
    }
}

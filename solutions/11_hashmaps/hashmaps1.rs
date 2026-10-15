// hashmaps1.rs - solution
//
// `insert` overwrites any existing value. `get` returns `Option<&String>`,
// and `map(|s| s.as_str())` (or `.map(String::as_str)`) turns that into
// `Option<&str>`. In `rename`, check everything *before* changing
// anything, so a failed rename leaves the book untouched. `remove` hands
// back the owned value, ready to re-insert under the new key.

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
        self.entries.insert(name.to_string(), number.to_string());
    }

    fn lookup(&self, name: &str) -> Option<&str> {
        self.entries.get(name).map(|s| s.as_str())
    }

    fn rename(&mut self, old: &str, new: &str) -> bool {
        if self.entries.contains_key(new) {
            return false;
        }
        match self.entries.remove(old) {
            Some(number) => {
                self.entries.insert(new.to_string(), number);
                true
            }
            None => false,
        }
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

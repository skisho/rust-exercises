// generics3.rs
//
// Build a generic `Stack<T>`:
//   - `new`, `push`, `pop` and `peek` work for *any* `T`
//   - `describe` only exists when `T` implements `Display`, and formats
//     the stack bottom-to-top like "[1, 2, 3]"
//
// That second part needs a separate `impl` block with a trait bound.

// I AM NOT DONE

use std::fmt::Display;

struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }

    fn push(&mut self, item: T) {
        todo!()
    }

    fn pop(&mut self) -> Option<T> {
        todo!()
    }

    fn peek(&self) -> Option<&T> {
        todo!()
    }
}

// TODO: an impl block where `describe` is only available for `T: Display`

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_pop_peek() {
        let mut s = Stack::new();
        s.push(1);
        s.push(2);
        assert_eq!(s.peek(), Some(&2));
        assert_eq!(s.pop(), Some(2));
        assert_eq!(s.pop(), Some(1));
        assert_eq!(s.pop(), None);
    }

    #[test]
    fn works_for_non_display_types() {
        let mut s = Stack::new();
        s.push(vec![1, 2]);
        assert_eq!(s.peek(), Some(&vec![1, 2]));
    }

    #[test]
    fn describe() {
        let mut s = Stack::new();
        for word in ["a", "b", "c"] {
            s.push(word);
        }
        assert_eq!(s.describe(), "[a, b, c]");
        assert_eq!(Stack::<i32>::new().describe(), "[]");
    }
}

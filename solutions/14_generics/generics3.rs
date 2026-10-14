// generics3.rs - solution
//
// You can have several `impl` blocks for one generic type. A block with a
// bound (`impl<T: Display>`) adds methods only for types that satisfy it:
// `Stack<Vec<i32>>` simply doesn't have `describe`.

use std::fmt::Display;

struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.items.last()
    }
}

impl<T: Display> Stack<T> {
    fn describe(&self) -> String {
        let parts: Vec<String> = self.items.iter().map(|item| item.to_string()).collect();
        format!("[{}]", parts.join(", "))
    }
}

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

// linked_list.rs (challenge)
//
// Build a singly linked stack: each node owns the next one through a Box.
//
//     head -> [3] -> [2] -> [1] -> None
//
// This is a famous Rust rite of passage. Linked lists fight the borrow
// checker, and winning teaches you a lot about ownership. The first two
// chapters of "Too Many Linked Lists" (linked below) walk through almost
// exactly this. Try it yourself first, then read them if you get stuck.
//
// Requirements:
//   1. `push`, `pop`, `peek`, `peek_mut`, `len`, `is_empty`.
//   2. `iter()` borrows the items top to bottom, without consuming the stack.
//   3. `IntoIterator` (for `for x in stack`) and `FromIterator` (for
//      `.collect()`). Collecting pushes each item, so the last becomes the top.
//   4. `reverse()` reverses the list in place, without allocating new nodes.
//   5. Implement `Drop`. The automatically generated drop is recursive
//      (each Box drops the next Box), so a list of a million nodes
//      overflows the stack. Write an iterative version instead.
//
// Key tools: `Option::take`, `Option::as_deref`, `Option::as_deref_mut`, `Option::map`.
//
// Further reading:
//   - Learning Rust With Entirely Too Many Linked Lists:
//     https://rust-unofficial.github.io/too-many-lists/
//     (chapter 2, "An Ok Singly-Linked Stack", is the most relevant:
//      https://rust-unofficial.github.io/too-many-lists/second.html)
//   - Option::take: https://doc.rust-lang.org/std/option/enum.Option.html#method.take
//   - Drop: https://doc.rust-lang.org/std/ops/trait.Drop.html
//   - IntoIterator: https://doc.rust-lang.org/std/iter/trait.IntoIterator.html
//   - FromIterator: https://doc.rust-lang.org/std/iter/trait.FromIterator.html

// I AM NOT DONE

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    value: T,
    next: Link<T>,
}

struct Stack<T> {
    head: Link<T>,
    len: usize,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { head: None, len: 0 }
    }

    fn push(&mut self, value: T) {
        todo!()
    }

    fn pop(&mut self) -> Option<T> {
        todo!()
    }

    fn peek(&self) -> Option<&T> {
        todo!()
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        todo!()
    }

    fn len(&self) -> usize {
        self.len
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn iter(&self) -> Iter<'_, T> {
        todo!()
    }

    fn reverse(&mut self) {
        todo!()
    }
}

/// Borrowing iterator: remembers which node comes next.
struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        todo!()
    }
}

// TODO: an owning iterator. A tuple struct wrapping the Stack is enough:
// struct IntoIter<T>(Stack<T>);
// impl<T> Iterator for IntoIter<T> { ... }
// impl<T> IntoIterator for Stack<T> { ... }

// TODO: impl<T> FromIterator<T> for Stack<T> { ... }

// TODO: impl<T> Drop for Stack<T> { ... }

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_pop() {
        let mut s = Stack::new();
        assert!(s.is_empty());
        assert_eq!(s.pop(), None);

        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.len(), 3);
        assert_eq!(s.pop(), Some(3));
        assert_eq!(s.pop(), Some(2));

        s.push(4);
        assert_eq!(s.pop(), Some(4));
        assert_eq!(s.pop(), Some(1));
        assert_eq!(s.pop(), None);
        assert!(s.is_empty());
    }

    #[test]
    fn peek() {
        let mut s = Stack::new();
        assert_eq!(s.peek(), None);
        s.push(String::from("a"));
        s.push(String::from("b"));
        assert_eq!(s.peek().map(String::as_str), Some("b"));

        if let Some(top) = s.peek_mut() {
            top.push('!');
        }
        assert_eq!(s.pop().as_deref(), Some("b!"));
    }

    #[test]
    fn iter_borrows_top_first() {
        let mut s = Stack::new();
        for n in 1..=3 {
            s.push(n);
        }
        let seen: Vec<&i32> = s.iter().collect();
        assert_eq!(seen, [&3, &2, &1]);
        assert_eq!(s.len(), 3, "iter must not consume the stack");
    }

    #[test]
    fn into_iter_and_for_loops() {
        let mut s = Stack::new();
        for n in 1..=3 {
            s.push(n);
        }
        let mut seen = Vec::new();
        for n in s {
            seen.push(n);
        }
        assert_eq!(seen, [3, 2, 1]);
    }

    #[test]
    fn collect_into_stack() {
        let s: Stack<char> = "abc".chars().collect();
        assert_eq!(s.peek(), Some(&'c'));
        assert_eq!(s.len(), 3);
    }

    #[test]
    fn reverse() {
        let mut s: Stack<i32> = (1..=4).collect();
        s.reverse();
        assert_eq!(s.into_iter().collect::<Vec<_>>(), [1, 2, 3, 4]);

        let mut empty: Stack<i32> = Stack::new();
        empty.reverse();
        assert!(empty.is_empty());
    }

    #[test]
    fn dropping_a_long_stack_does_not_overflow() {
        let mut s = Stack::new();
        for n in 0..1_000_000 {
            s.push(n);
        }
        drop(s);
    }
}

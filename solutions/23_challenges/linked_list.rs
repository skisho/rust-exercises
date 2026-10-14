// linked_list.rs (challenge) - solution
//
// The trick behind almost every method is `Option::take`, which moves the
// value out and leaves `None` behind. That lets you take ownership of a
// node out of a `&mut` without breaking the list in the meantime.
//
// `as_deref()` turns `&Option<Box<Node>>` into `Option<&Node>`, which is
// exactly what the borrowing iterator needs.
//
// Drop: pulling each node's `next` out *before* the node is dropped means
// no Box ever has a chain hanging off it, so nothing recurses.

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
        let node = Box::new(Node { value, next: self.head.take() });
        self.head = Some(node);
        self.len += 1;
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.len -= 1;
            node.value
        })
    }

    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.value)
    }

    fn len(&self) -> usize {
        self.len
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_deref() }
    }

    fn reverse(&mut self) {
        let mut reversed: Link<T> = None;
        let mut current = self.head.take();
        while let Some(mut node) = current {
            current = node.next.take();
            node.next = reversed;
            reversed = Some(node);
        }
        self.head = reversed;
    }
}

struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.value
        })
    }
}

struct IntoIter<T>(Stack<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.0.pop()
    }
}

impl<T> IntoIterator for Stack<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> FromIterator<T> for Stack<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut stack = Stack::new();
        for item in iter {
            stack.push(item);
        }
        stack
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(mut node) = current {
            current = node.next.take();
            // `node` is dropped here, with nothing left attached to it.
        }
    }
}

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

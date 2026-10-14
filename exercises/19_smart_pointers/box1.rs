// box1.rs
//
// A cons list is a classic recursive data structure: each item holds a
// value and the rest of the list. As written, `List` has infinite size,
// so it doesn't compile. Use `Box` to fix it, then implement
// `create_list`.

// I AM NOT DONE

#[derive(PartialEq, Debug)]
enum List {
    Cons(i32, List),
    Nil,
}

use List::{Cons, Nil};

fn create_empty_list() -> List {
    Nil
}

// Should build the list 1 -> 2 -> 3 -> Nil
fn create_list() -> List {
    todo!()
}

fn main() {
    println!("{:?}", create_list());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_list() {
        assert_eq!(create_empty_list(), Nil);
    }

    #[test]
    fn non_empty_list() {
        let list = create_list();
        assert_ne!(list, Nil);
        if let Cons(first, _) = list {
            assert_eq!(first, 1);
        }
    }
}

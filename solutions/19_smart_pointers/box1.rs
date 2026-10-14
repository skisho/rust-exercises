// box1.rs - solution
//
// A `Box<T>` is a pointer to heap data, and a pointer has a known, fixed
// size. That breaks the infinite recursion in the type's size.

#[derive(PartialEq, Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn create_empty_list() -> List {
    Nil
}

fn create_list() -> List {
    Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))))
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

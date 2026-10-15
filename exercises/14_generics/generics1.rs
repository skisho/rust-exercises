// generics1.rs
//
// `Pair` only holds a `String` and an `i32`. Make it generic over *two*
// type parameters, so the first and second values can each be any type.
//
// `swap` returns a new pair with the values the other way round. Think
// about what its return type should be once `Pair` is generic.

// I AM NOT DONE

#[derive(Debug, PartialEq)]
struct Pair {
    first: String,
    second: i32,
}

impl Pair {
    fn new(first: String, second: i32) -> Self {
        Pair { first, second }
    }

    fn swap(self) -> Pair {
        todo!()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mixed_types() {
        let p = Pair::new("answer", 42);
        assert_eq!(p.first, "answer");
        assert_eq!(p.second, 42);
    }

    #[test]
    fn same_types() {
        let p = Pair::new(1.5, 2.5);
        assert_eq!(p.first + p.second, 4.0);
    }

    #[test]
    fn swapping_changes_the_type() {
        let p = Pair::new('x', true);
        let swapped: Pair<bool, char> = p.swap();
        assert_eq!(swapped, Pair::new(true, 'x'));
    }
}

// generics1.rs - solution
//
// `impl<A, B> Pair<A, B>` declares the type parameters for the whole
// block. `swap` returns `Pair<B, A>`: the parameters trade places, so a
// `Pair<char, bool>` becomes a `Pair<bool, char>`. No trait bounds are
// needed, because the code only moves the values around.

#[derive(Debug, PartialEq)]
struct Pair<A, B> {
    first: A,
    second: B,
}

impl<A, B> Pair<A, B> {
    fn new(first: A, second: B) -> Self {
        Pair { first, second }
    }

    fn swap(self) -> Pair<B, A> {
        Pair {
            first: self.second,
            second: self.first,
        }
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

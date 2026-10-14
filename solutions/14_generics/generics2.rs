// generics2.rs - solution
//
// The `>` operator comes from the `PartialOrd` trait. A generic function
// can only use what its bounds promise, so `T` must be `PartialOrd`.

fn largest<T: PartialOrd>(items: &[T]) -> &T {
    let mut largest = &items[0];
    for item in items {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers() {
        assert_eq!(*largest(&[34, 50, 25, 100, 65]), 100);
    }

    #[test]
    fn chars() {
        assert_eq!(*largest(&['y', 'm', 'a', 'q']), 'y');
    }

    #[test]
    fn strings() {
        assert_eq!(*largest(&["apple", "pear", "fig"]), "pear");
    }
}

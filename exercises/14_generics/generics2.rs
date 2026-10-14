// generics2.rs
//
// `largest` should work on a slice of anything that can be compared.
// Make it compile by adding a trait bound to `T`.

// I AM NOT DONE

fn largest<T>(items: &[T]) -> &T {
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

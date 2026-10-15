// generics2.rs
//
// `range_label` describes the spread of some values: for [4, 9, 1] it
// returns "1 to 9". It should work for any type that can be *compared*
// and *displayed*.
//
// The body is finished. Make it compile by adding the trait bounds `T`
// needs. Try writing them in a `where` clause.

// I AM NOT DONE

use std::fmt::Display;

fn range_label<T>(items: &[T]) -> String {
    let Some(first) = items.first() else {
        return String::from("nothing");
    };

    let mut low = first;
    let mut high = first;
    for item in items {
        if item < low {
            low = item;
        }
        if item > high {
            high = item;
        }
    }
    format!("{low} to {high}")
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers() {
        assert_eq!(range_label(&[4, 9, 1]), "1 to 9");
    }

    #[test]
    fn floats() {
        assert_eq!(range_label(&[2.5, -0.5]), "-0.5 to 2.5");
    }

    #[test]
    fn words() {
        assert_eq!(range_label(&["pear", "apple", "fig"]), "apple to pear");
    }

    #[test]
    fn empty() {
        let none: [u8; 0] = [];
        assert_eq!(range_label(&none), "nothing");
    }
}

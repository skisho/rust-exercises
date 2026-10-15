// generics2.rs - solution
//
// `<` and `>` come from `PartialOrd`, and `{}` needs `Display`. A `where`
// clause lists the bounds after the signature, which reads better once
// there are several. `fn range_label<T: PartialOrd + Display>` is the
// same thing written inline.

use std::fmt::Display;

fn range_label<T>(items: &[T]) -> String
where
    T: PartialOrd + Display,
{
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

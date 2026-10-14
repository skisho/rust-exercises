// traits4.rs
//
// Many common traits can be derived automatically, and some must be
// written by hand. Make the tests compile and pass:
//   - derive the traits the tests need (Debug, Clone, PartialEq, Default)
//   - implement `Display` by hand so a point prints as "(x, y)"

// I AM NOT DONE

use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let p = Point { x: 3, y: -4 };
        assert_eq!(p.to_string(), "(3, -4)");
        assert_eq!(format!("at {p}"), "at (3, -4)");
    }

    #[test]
    fn derived() {
        let p = Point { x: 1, y: 2 };
        let q = p.clone();
        assert_eq!(p, q);
        assert_eq!(format!("{p:?}"), "Point { x: 1, y: 2 }");
        assert_eq!(Point::default(), Point { x: 0, y: 0 });
    }
}

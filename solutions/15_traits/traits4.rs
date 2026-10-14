// traits4.rs - solution
//
// `#[derive(...)]` generates the obvious implementation field by field.
// `Display` is never derived, because there's no single obvious
// user-facing format. Implementing it also gives you `.to_string()`.

use std::fmt;

#[derive(Debug, Clone, PartialEq, Default)]
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
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

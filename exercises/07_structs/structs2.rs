// structs2.rs
//
// Add an `impl` block for `Rectangle` with:
//   - an associated function `new(width, height) -> Rectangle`
//   - a method `area(&self) -> u32`
//   - a method `can_hold(&self, other: &Rectangle) -> bool` that returns
//     true if `other` fits entirely inside `self`

// I AM NOT DONE

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// TODO: impl Rectangle { ... }

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area() {
        let r = Rectangle::new(3, 4);
        assert_eq!(r.area(), 12);
    }

    #[test]
    fn can_hold() {
        let big = Rectangle::new(10, 10);
        let small = Rectangle::new(2, 5);
        assert!(big.can_hold(&small));
        assert!(!small.can_hold(&big));
    }
}

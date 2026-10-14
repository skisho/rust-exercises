// structs2.rs - solution
//
// Functions in an `impl` block without `self` are "associated functions"
// (called with `Type::name`). Ones taking `&self` are methods.

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

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

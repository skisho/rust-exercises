// traits5.rs
//
// Operators like `+` and `*` are just traits from `std::ops`. Implement:
//   - `Add` so two `Vec2`s can be added
//   - `Mul<f64>` so a `Vec2` can be scaled by a number
//
// Both traits have an associated type, `Output`, saying what the
// operation produces.

// I AM NOT DONE

use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec2 {
    x: f64,
    y: f64,
}

// TODO: impl Add for Vec2 { ... }

// TODO: impl Mul<f64> for Vec2 { ... }

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let a = Vec2 { x: 1.0, y: 2.0 };
        let b = Vec2 { x: 0.5, y: -1.0 };
        assert_eq!(a + b, Vec2 { x: 1.5, y: 1.0 });
    }

    #[test]
    fn scale() {
        let a = Vec2 { x: 1.0, y: -2.0 };
        assert_eq!(a * 3.0, Vec2 { x: 3.0, y: -6.0 });
    }

    #[test]
    fn combined() {
        let pos = Vec2 { x: 0.0, y: 0.0 };
        let velocity = Vec2 { x: 2.0, y: 1.0 };
        assert_eq!(pos + velocity * 0.5, Vec2 { x: 1.0, y: 0.5 });
    }
}

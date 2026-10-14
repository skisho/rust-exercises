// traits5.rs - solution
//
// `a + b` is sugar for `Add::add(a, b)`. The `Output` associated type
// lets the result differ from the inputs. `Mul<f64>` names the type of
// the right-hand side: a Vec2 times an f64.

use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2 { x: self.x + other.x, y: self.y + other.y }
    }
}

impl Mul<f64> for Vec2 {
    type Output = Vec2;

    fn mul(self, factor: f64) -> Vec2 {
        Vec2 { x: self.x * factor, y: self.y * factor }
    }
}

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

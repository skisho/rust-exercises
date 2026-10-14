// enums2.rs - solution
//
// `match` must be exhaustive: the compiler makes you handle every variant.

use std::f64::consts::PI;

enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle { base, height } => 0.5 * base * height,
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circle() {
        let c = Shape::Circle { radius: 1.0 };
        assert!((c.area() - PI).abs() < 1e-9);
    }

    #[test]
    fn rectangle() {
        let r = Shape::Rectangle { width: 3.0, height: 4.0 };
        assert_eq!(r.area(), 12.0);
    }

    #[test]
    fn triangle() {
        let t = Shape::Triangle { base: 6.0, height: 2.0 };
        assert_eq!(t.area(), 6.0);
    }
}

// traits3.rs
//
// We want one Vec holding *different* shape types. A Vec can only hold
// one type, so use trait objects: values behind a pointer whose concrete
// type is only known at runtime.
//
// Fix `total_area`'s parameter type and the `shapes` Vec in the test.

// I AM NOT DONE

trait Shape {
    fn area(&self) -> f64;
}

struct Square {
    side: f64,
}

struct Circle {
    radius: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

fn total_area(shapes: &[Shape]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mixed_shapes() {
        let shapes = vec![
            Square { side: 2.0 },
            Circle { radius: 1.0 },
            Square { side: 1.0 },
        ];
        let expected = 4.0 + std::f64::consts::PI + 1.0;
        assert!((total_area(&shapes) - expected).abs() < 1e-9);
    }
}

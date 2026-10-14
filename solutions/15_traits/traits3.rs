// traits3.rs - solution
//
// `dyn Shape` has no fixed size, so it must live behind a pointer such
// as `Box<dyn Shape>` or `&dyn Shape`. Calls go through a vtable chosen
// at runtime ("dynamic dispatch"), unlike generics, which are resolved
// at compile time.

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

fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mixed_shapes() {
        let shapes: Vec<Box<dyn Shape>> = vec![
            Box::new(Square { side: 2.0 }),
            Box::new(Circle { radius: 1.0 }),
            Box::new(Square { side: 1.0 }),
        ];
        let expected = 4.0 + std::f64::consts::PI + 1.0;
        assert!((total_area(&shapes) - expected).abs() < 1e-9);
    }
}

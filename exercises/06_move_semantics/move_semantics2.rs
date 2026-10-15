// move_semantics2.rs
//
// `distance_from_origin` takes a `Point` by value, so calling it moves the
// point, and the second call fails to compile.
//
// This time, don't change the function or the test. Points are small and
// made entirely of numbers, so let them be *copied* instead of moved.
// (You'll need to add one line.)
//
// Then think: could `Label` below get the same treatment? Why not?

// I AM NOT DONE

#[derive(Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

#[allow(dead_code)]
struct Label {
    text: String,
}

fn distance_from_origin(p: Point) -> f64 {
    (p.x * p.x + p.y * p.y).sqrt()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn use_after_passing() {
        let p = Point { x: 3.0, y: 4.0 };
        assert_eq!(distance_from_origin(p), 5.0);
        assert_eq!(distance_from_origin(p), 5.0);
        assert_eq!(p, Point { x: 3.0, y: 4.0 });
    }
}

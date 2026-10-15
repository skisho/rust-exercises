// move_semantics2.rs - solution
//
// Types that implement `Copy` are duplicated bit-for-bit when passed by
// value, so the original stays usable. `Copy` requires `Clone`, and you
// can only derive it when every field is `Copy` too.
//
// `Label` can't be `Copy`: it contains a `String`, which owns heap memory.
// Copying it bit-for-bit would leave two owners of one allocation, and
// both would try to free it. For types like that, you call `.clone()`
// explicitly so the cost is visible.

#[derive(Debug, Clone, Copy, PartialEq)]
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

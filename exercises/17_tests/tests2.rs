// tests2.rs
//
// `Rectangle::new` panics if given a negative width or height. The
// second and third tests check this, but they fail because the panic is
// treated as a test failure. Tell the test harness a panic is expected.

// I AM NOT DONE

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn new(width: i32, height: i32) -> Self {
        if width < 0 || height < 0 {
            panic!("Rectangle width and height can't be negative");
        }
        Rectangle { width, height }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10);
        assert_eq!(rect.height, 20);
    }

    #[test]
    fn negative_width() {
        let _rect = Rectangle::new(-10, 10);
    }

    #[test]
    fn negative_height() {
        let _rect = Rectangle::new(10, -10);
    }
}

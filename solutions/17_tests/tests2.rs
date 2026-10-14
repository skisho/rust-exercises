// tests2.rs - solution
//
// `#[should_panic]` makes a test pass only if it panics. Adding
// `expected = "..."` also checks the panic message contains that text.

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
    #[should_panic(expected = "can't be negative")]
    fn negative_width() {
        let _rect = Rectangle::new(-10, 10);
    }

    #[test]
    #[should_panic(expected = "can't be negative")]
    fn negative_height() {
        let _rect = Rectangle::new(10, -10);
    }
}

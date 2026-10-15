// functions1.rs
//
// Make the tests pass. `rectangle_area` is called with a width *and* a
// height, but it was written for squares. Fix the function, not the tests.

// I AM NOT DONE

fn rectangle_area(side: u32) -> u32 {
    side * side
}

fn main() {
    println!("a 3x4 rug covers {} square metres", rectangle_area(3, 4));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rug() {
        assert_eq!(rectangle_area(3, 4), 12);
    }

    #[test]
    fn square_still_works() {
        assert_eq!(rectangle_area(5, 5), 25);
    }

    #[test]
    fn zero_width() {
        assert_eq!(rectangle_area(0, 9), 0);
    }
}

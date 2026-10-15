// functions1.rs - solution
//
// Parameters are a comma-separated list, and every one needs its own type.
// The call site and the definition must agree on how many there are: Rust
// has no default or optional arguments.

fn rectangle_area(width: u32, height: u32) -> u32 {
    width * height
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

// primitive_types2.rs
//
// A week of daily temperatures is stored in a fixed-size array, `[i32; 7]`.
// Use arrays, slices and tuples to make the tests pass:
//   - `weekend` returns a slice of the last two days
//   - `extremes` returns the (coldest, warmest) temperatures as a tuple
//   - fill in the `???`s in the tests

// I AM NOT DONE

fn weekend(week: &[i32; 7]) -> &[i32] {
    todo!()
}

fn extremes(week: &[i32; 7]) -> (i32, i32) {
    todo!()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    const WEEK: [i32; 7] = [12, 15, 9, 11, 18, 21, 17];

    #[test]
    fn the_weekend() {
        assert_eq!(weekend(&WEEK), [21, 17]);
    }

    #[test]
    fn coldest_and_warmest() {
        // TODO: destructure the tuple returned by `extremes` into `low`
        // and `high` with a single `let`.
        ???
        assert_eq!(low, 9);
        assert_eq!(high, 21);
    }

    #[test]
    fn a_week_of_frost() {
        // TODO: make an array of seven -3s without typing -3 seven times.
        let frost: [i32; 7] = ???;
        assert_eq!(extremes(&frost), (-3, -3));
        assert_eq!(weekend(&frost), [-3, -3]);
    }
}

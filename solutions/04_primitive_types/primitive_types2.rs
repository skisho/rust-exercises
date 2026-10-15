// primitive_types2.rs - solution
//
// `&week[5..]` borrows from index 5 to the end. A slice (`&[i32]`) doesn't
// know its length at compile time, unlike the array (`[i32; 7]`).
//
// Tuples are the easy way to return several values, and `let (a, b) = ...`
// unpacks them. `[value; count]` builds an array by repetition.

fn weekend(week: &[i32; 7]) -> &[i32] {
    &week[5..]
}

fn extremes(week: &[i32; 7]) -> (i32, i32) {
    let mut low = week[0];
    let mut high = week[0];
    for &t in week {
        if t < low {
            low = t;
        }
        if t > high {
            high = t;
        }
    }
    (low, high)
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
        let (low, high) = extremes(&WEEK);
        assert_eq!(low, 9);
        assert_eq!(high, 21);
    }

    #[test]
    fn a_week_of_frost() {
        let frost: [i32; 7] = [-3; 7];
        assert_eq!(extremes(&frost), (-3, -3));
        assert_eq!(weekend(&frost), [-3, -3]);
    }
}

// vecs2.rs
//
// Implement both functions so every element of the Vec is doubled.
// The first should modify the Vec in place, the second should build
// and return a new Vec.

// I AM NOT DONE

fn double_in_place(v: &mut Vec<i32>) {
    for element in v.iter_mut() {
        // TODO: double the value `element` points at.
    }
}

fn doubled(v: &[i32]) -> Vec<i32> {
    // TODO: return a new Vec with each element doubled.
    // Hint: `v.iter().map(...)` followed by `.collect()`.
    Vec::new()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_place() {
        let mut v = vec![1, 2, 3, 4];
        double_in_place(&mut v);
        assert_eq!(v, [2, 4, 6, 8]);
    }

    #[test]
    fn new_vec() {
        let v = vec![1, 2, 3, 4];
        assert_eq!(doubled(&v), [2, 4, 6, 8]);
        assert_eq!(v, [1, 2, 3, 4], "the original must be unchanged");
    }
}

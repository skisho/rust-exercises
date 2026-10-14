// primitive_types1.rs
//
// Arrays have a fixed length; slices are a view into part of one.
// Make the tests pass by filling in the TODOs.

// I AM NOT DONE

fn main() {}

#[cfg(test)]
mod tests {
    #[test]
    fn array_of_zeros() {
        // TODO: create an array of 100 zeros *without* typing them all out.
        let zeros = ???;
        assert_eq!(zeros.len(), 100);
        assert!(zeros.iter().all(|&n| n == 0));
    }

    #[test]
    fn middle_slice() {
        let numbers = [1, 2, 3, 4, 5];
        // TODO: take a slice of `numbers` so the assertion passes.
        let middle: &[i32] = ???;
        assert_eq!(middle, [2, 3, 4]);
    }
}

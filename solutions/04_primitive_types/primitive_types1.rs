// primitive_types1.rs - solution
//
// `[value; count]` repeats a value. `&array[start..end]` borrows a slice
// covering indices `start` up to (but not including) `end`.

fn main() {}

#[cfg(test)]
mod tests {
    #[test]
    fn array_of_zeros() {
        let zeros = [0; 100];
        assert_eq!(zeros.len(), 100);
        assert!(zeros.iter().all(|&n| n == 0));
    }

    #[test]
    fn middle_slice() {
        let numbers = [1, 2, 3, 4, 5];
        let middle: &[i32] = &numbers[1..4];
        assert_eq!(middle, [2, 3, 4]);
    }
}

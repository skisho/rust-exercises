// move_semantics4.rs
//
// You can have *either* one mutable reference *or* any number of shared
// references to a value at the same time, never both. Reorder the lines
// in the test (don't add or remove any) so it compiles.

// I AM NOT DONE

fn main() {}

#[cfg(test)]
mod tests {
    #[test]
    fn borrows_do_not_overlap() {
        let mut scores = vec![10, 20];

        let first = &scores[0];
        scores.push(30);
        assert_eq!(*first, 10);

        assert_eq!(scores, [10, 20, 30]);
    }
}

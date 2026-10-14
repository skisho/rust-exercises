// move_semantics4.rs - solution
//
// `push` needs `&mut scores`, but `first` is still a live shared borrow
// (it's used afterwards). A borrow only lasts until its last use, so
// finishing with `first` before pushing fixes it. This rule isn't
// pedantry: `push` may reallocate the Vec, leaving `first` dangling.

fn main() {}

#[cfg(test)]
mod tests {
    #[test]
    fn borrows_do_not_overlap() {
        let mut scores = vec![10, 20];

        let first = &scores[0];
        assert_eq!(*first, 10);
        scores.push(30);

        assert_eq!(scores, [10, 20, 30]);
    }
}

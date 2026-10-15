// variables4.rs - solution
//
// `level` goes 1 -> 2. Inside the block a new `level` (20) hides the outer
// one, and the block evaluates to 25. When the block ends, the outer
// `level` (still 2) is visible again, and the last shadow makes it 1.
//
// In the second test each `let` creates a variable of a different type:
// &str -> &str -> u32 -> bool. That's something `mut` can't do.

fn main() {}

#[cfg(test)]
mod tests {
    #[test]
    fn predict_the_values() {
        let level = 1;
        let level = level + 1;

        let inner = {
            let level = level * 10;
            level + 5
        };

        let level = level - 1;

        assert_eq!(inner, 25);
        assert_eq!(level, 1);
    }

    #[test]
    fn shadowing_can_change_the_type() {
        let answer = "  17 ";
        let answer = answer.trim();
        let answer: u32 = answer.parse().unwrap();
        let answer = answer > 10;

        assert_eq!(answer, true);
    }
}

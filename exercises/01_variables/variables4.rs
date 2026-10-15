// variables4.rs
//
// Shadowing: a new `let` with an existing name creates a *new* variable
// that hides the old one until the end of its block.
//
// Don't change any of the code above the assertions. Instead, work out
// what each value is and replace every `???` with it.

// I AM NOT DONE

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

        assert_eq!(inner, ???);
        assert_eq!(level, ???);
    }

    #[test]
    fn shadowing_can_change_the_type() {
        let answer = "  17 ";
        let answer = answer.trim();
        let answer: u32 = answer.parse().unwrap();
        let answer = answer > 10;

        assert_eq!(answer, ???);
    }
}

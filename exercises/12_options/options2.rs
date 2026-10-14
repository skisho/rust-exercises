// options2.rs
//
// Make the tests pass using `if let` and `while let`.

// I AM NOT DONE

fn main() {}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustacean";
        let optional_target = Some(target);

        // TODO: make this an `if let` statement whose value is `Some(word)`
        word = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn drain_the_stack() {
        let mut stack = vec![1, 2, 3];
        let mut popped = Vec::new();

        // TODO: make this a `while let` that keeps going while `pop()`
        // returns `Some`. Push each value into `popped`.
        integer = stack.pop() {
            popped.push(integer);
        }

        assert_eq!(popped, [3, 2, 1]);
        assert!(stack.is_empty());
    }
}

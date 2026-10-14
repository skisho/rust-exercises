// options2.rs - solution
//
// `if let PATTERN = expr` runs its block only when the pattern matches.
// `while let` loops for as long as the pattern keeps matching.

fn main() {}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustacean";
        let optional_target = Some(target);

        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn drain_the_stack() {
        let mut stack = vec![1, 2, 3];
        let mut popped = Vec::new();

        while let Some(integer) = stack.pop() {
            popped.push(integer);
        }

        assert_eq!(popped, [3, 2, 1]);
        assert!(stack.is_empty());
    }
}

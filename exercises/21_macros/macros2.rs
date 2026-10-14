// macros2.rs
//
// Write a `max!` macro that accepts one or more expressions separated by
// commas and returns the largest. With one argument it returns that
// argument.
//
// Hint: you'll need two rules: one for a single expression, and one that
// handles "first expression, then the rest" recursively.

// I AM NOT DONE

macro_rules! max {
    // TODO
}

fn main() {}

#[cfg(test)]
mod tests {
    #[test]
    fn single() {
        assert_eq!(max!(7), 7);
    }

    #[test]
    fn several() {
        assert_eq!(max!(3, 9, 4), 9);
        assert_eq!(max!(1, 2, 3, 4, 5, 2), 5);
    }

    #[test]
    fn expressions() {
        assert_eq!(max!(2 * 5, 3 + 3), 10);
    }
}

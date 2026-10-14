// strings2.rs
//
// Implement each function using methods on `str` and `String`.
// The standard library docs for `str` are your friend here.

// I AM NOT DONE

fn trim_me(input: &str) -> &str {
    // TODO: remove whitespace from both ends
    input
}

fn compose_me(input: &str) -> String {
    // TODO: add " world!" to the end of the input
    input.to_string()
}

fn replace_me(input: &str) -> String {
    // TODO: replace every "cars" with "balloons"
    input.to_string()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim() {
        assert_eq!(trim_me("  Hello!  "), "Hello!");
        assert_eq!(trim_me("\tWhat's up!\n"), "What's up!");
    }

    #[test]
    fn compose() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool"
        );
        assert_eq!(
            replace_me("I love to look at cars, cars, cars"),
            "I love to look at balloons, balloons, balloons"
        );
    }
}

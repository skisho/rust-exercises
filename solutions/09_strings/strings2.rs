// strings2.rs - solution
//
// `trim` returns a sub-slice of the input, so no allocation is needed.
// `format!` (or `String + &str`) builds a new String; `replace` too.

fn trim_me(input: &str) -> &str {
    input.trim()
}

fn compose_me(input: &str) -> String {
    format!("{input} world!")
}

fn replace_me(input: &str) -> String {
    input.replace("cars", "balloons")
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

// cow1.rs
//
// `Cow` ("clone on write") holds either borrowed data or owned data.
// It's handy when a function *usually* returns its input unchanged but
// *sometimes* has to modify it: you only pay for an allocation when
// something actually changes.
//
// Implement `tabs_to_spaces`: if the input has no tabs, return it
// borrowed. Otherwise return an owned String with each tab replaced by
// four spaces.

// I AM NOT DONE

use std::borrow::Cow;

fn tabs_to_spaces(input: &str) -> Cow<'_, str> {
    todo!()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unchanged_is_borrowed() {
        let result = tabs_to_spaces("no tabs here");
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(result, "no tabs here");
    }

    #[test]
    fn changed_is_owned() {
        let result = tabs_to_spaces("\tindented");
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result, "    indented");
    }
}

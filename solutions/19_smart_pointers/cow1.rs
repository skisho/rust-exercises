// cow1.rs - solution
//
// `Cow::Borrowed` wraps the input reference, `Cow::Owned` wraps a new
// String. Either way, callers can use the result like a `&str`.

use std::borrow::Cow;

fn tabs_to_spaces(input: &str) -> Cow<'_, str> {
    if input.contains('\t') {
        Cow::Owned(input.replace('\t', "    "))
    } else {
        Cow::Borrowed(input)
    }
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

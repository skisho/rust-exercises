// options2.rs
//
// Matching on every Option gets tedious. Option has many helper methods
// ("combinators") that do the common things in one call. Implement each
// function in one expression, using the method named in its comment.
// No `match` or `if let` allowed!

// I AM NOT DONE

// The display name is the nickname if there is one, otherwise the real
// name.                                                       (unwrap_or)
fn display_name<'a>(real_name: &'a str, nickname: Option<&'a str>) -> &'a str {
    todo!()
}

// The length of the middle name, if there is one.                   (map)
fn middle_name_length(middle: Option<&str>) -> Option<usize> {
    todo!()
}

// Parse the port setting, if there is one. A setting that is missing
// *or* invalid gives None.                                     (and_then)
// Hint: `s.parse::<u16>().ok()` turns a parse Result into an Option.
fn port(setting: Option<&str>) -> Option<u16> {
    todo!()
}

// Keep the score only if it is a new high score (strictly greater than
// `best`).                                                       (filter)
fn new_high_score(score: Option<u32>, best: u32) -> Option<u32> {
    todo!()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names() {
        assert_eq!(display_name("Ferris Crab", Some("Ferris")), "Ferris");
        assert_eq!(display_name("Ferris Crab", None), "Ferris Crab");
    }

    #[test]
    fn middle_names() {
        assert_eq!(middle_name_length(Some("Brewster")), Some(8));
        assert_eq!(middle_name_length(None), None);
    }

    #[test]
    fn ports() {
        assert_eq!(port(Some("8080")), Some(8080));
        assert_eq!(port(Some("http")), None);
        assert_eq!(port(Some("99999")), None);
        assert_eq!(port(None), None);
    }

    #[test]
    fn high_scores() {
        assert_eq!(new_high_score(Some(120), 100), Some(120));
        assert_eq!(new_high_score(Some(100), 100), None);
        assert_eq!(new_high_score(None, 100), None);
    }
}

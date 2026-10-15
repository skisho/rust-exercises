// options2.rs - solution
//
// - `unwrap_or(default)` gets the value out, falling back to a default.
// - `map(f)` transforms the value inside a Some and leaves None alone.
// - `and_then(f)` is for when `f` *itself* returns an Option; `map` would
//   give an awkward `Option<Option<u16>>`.
// - `filter(pred)` turns a Some into None when the predicate fails.
//
// The `'a` on `display_name` says the result borrows from one of the two
// inputs (lifetimes are covered properly in topic 16).

fn display_name<'a>(real_name: &'a str, nickname: Option<&'a str>) -> &'a str {
    nickname.unwrap_or(real_name)
}

fn middle_name_length(middle: Option<&str>) -> Option<usize> {
    middle.map(|m| m.len())
}

fn port(setting: Option<&str>) -> Option<u16> {
    setting.and_then(|s| s.parse::<u16>().ok())
}

fn new_high_score(score: Option<u32>, best: u32) -> Option<u32> {
    score.filter(|&s| s > best)
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

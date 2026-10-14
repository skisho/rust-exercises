// lifetimes2.rs
//
// `Excerpt` holds a reference into some text rather than owning a copy.
// Add the lifetime annotations it needs.

// I AM NOT DONE

struct Excerpt {
    part: &str,
}

impl Excerpt {
    fn first_sentence(text: &str) -> Excerpt {
        let part = text.split('.').next().unwrap_or("");
        Excerpt { part }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_sentence() {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let excerpt = Excerpt::first_sentence(&novel);
        assert_eq!(excerpt.part, "Call me Ishmael");
    }
}

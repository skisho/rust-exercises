// lifetimes1.rs
//
// `strip_tag` removes a prefix like "user:" from some text and returns
// the rest. The result always points into `text`, never into `tag`.
//
// With two reference parameters, the compiler can't guess which one the
// result borrows from, so you have to say. Add lifetime annotations, and
// get them *exactly* right: the second test keeps the result after `tag`
// has been dropped.

// I AM NOT DONE

fn strip_tag(text: &str, tag: &str) -> &str {
    text.strip_prefix(tag).unwrap_or(text)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips() {
        assert_eq!(strip_tag("user:ferris", "user:"), "ferris");
        assert_eq!(strip_tag("admin:root", "user:"), "admin:root");
    }

    #[test]
    fn result_outlives_the_tag() {
        let text = String::from("user:ferris");
        let name;
        {
            let tag = String::from("user:");
            name = strip_tag(&text, &tag);
        }
        assert_eq!(name, "ferris");
    }
}

// lifetimes1.rs - solution
//
// `'a` connects the output to `text` only. `tag` gets no named lifetime,
// which tells the compiler the result never borrows from it, so `tag` is
// free to be dropped early.
//
// Writing `'a` on all three (as in the classic `longest` example) would
// also compile `strips`, but would break the second test: the compiler
// would then assume `name` might point into `tag`.

fn strip_tag<'a>(text: &'a str, tag: &str) -> &'a str {
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

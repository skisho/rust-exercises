// lifetimes2.rs - solution
//
// A struct holding a reference needs a lifetime parameter: an `Excerpt`
// can't outlive the text it points into.

struct Excerpt<'a> {
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    fn first_sentence(text: &'a str) -> Excerpt<'a> {
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

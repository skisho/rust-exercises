// traits2.rs - solution
//
// Trait methods can have default bodies that call other trait methods.
// `&impl Summary` is shorthand for a generic `<T: Summary>(item: &T)`.

trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

struct Article {
    author: String,
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for Article {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.summarize_author(), self.content)
    }
}

fn notify(item: &impl Summary) -> String {
    format!("Breaking news! {}", item.summarize())
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn article_uses_default() {
        let a = Article { author: "Ferris".to_string() };
        assert_eq!(a.summarize(), "(Read more from Ferris...)");
    }

    #[test]
    fn tweet_overrides() {
        let t = Tweet { username: "rustlang".to_string(), content: "1.0 is out!".to_string() };
        assert_eq!(notify(&t), "Breaking news! @rustlang: 1.0 is out!");
    }
}

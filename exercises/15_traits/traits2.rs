// traits2.rs
//
// 1. Give `Summary::summarize` a default implementation that returns
//    "(Read more from {author}...)", using `summarize_author`.
// 2. `Tweet` should override `summarize`, but `Article` should use the
//    default.
// 3. Make `notify` accept anything that implements `Summary`.

// I AM NOT DONE

trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String;
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

fn notify(item: ???) -> String {
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

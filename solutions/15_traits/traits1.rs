// traits1.rs - solution
//
// Each `impl Emoji for X` block provides the trait's methods for one type.
// You may implement a trait for a type as long as either the trait or the
// type is defined in your crate (the "orphan rule"). `Emoji` is ours, so
// implementing it for `bool` is allowed.
//
// The `T: Emoji` bound lets `forecast` call `.emoji()` on any such type.

trait Emoji {
    fn emoji(&self) -> char;
}

enum Weather {
    Sunny,
    Rainy,
    Snowy,
}

impl Emoji for Weather {
    fn emoji(&self) -> char {
        match self {
            Weather::Sunny => '☀',
            Weather::Rainy => '🌧',
            Weather::Snowy => '❄',
        }
    }
}

impl Emoji for bool {
    fn emoji(&self) -> char {
        if *self { '✅' } else { '❌' }
    }
}

fn forecast<T: Emoji>(items: &[T]) -> String {
    items.iter().map(|item| item.emoji()).collect()
}

fn main() {
    println!("{}", forecast(&[Weather::Sunny, Weather::Rainy]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weather() {
        assert_eq!(Weather::Sunny.emoji(), '☀');
        assert_eq!(Weather::Snowy.emoji(), '❄');
    }

    #[test]
    fn booleans() {
        assert_eq!(true.emoji(), '✅');
        assert_eq!(false.emoji(), '❌');
    }

    #[test]
    fn week_ahead() {
        let week = [Weather::Rainy, Weather::Rainy, Weather::Sunny, Weather::Snowy];
        assert_eq!(forecast(&week), "🌧🌧☀❄");
        assert_eq!(forecast(&[true, false, true]), "✅❌✅");
    }
}

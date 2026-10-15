// traits1.rs
//
// A trait describes behaviour that many types can share. Implement the
// `Emoji` trait for:
//   - `Weather`: Sunny -> '☀', Rainy -> '🌧', Snowy -> '❄'
//   - `bool`:    true -> '✅', false -> '❌'
//
// Yes, you can implement your own traits for built-in types like `bool`!
// Then write `forecast`, which turns a list of anything that implements
// `Emoji` into a String of emoji.

// I AM NOT DONE

trait Emoji {
    fn emoji(&self) -> char;
}

enum Weather {
    Sunny,
    Rainy,
    Snowy,
}

// TODO: impl Emoji for Weather { ... }

// TODO: impl Emoji for bool { ... }

fn forecast<T: Emoji>(items: &[T]) -> String {
    todo!()
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

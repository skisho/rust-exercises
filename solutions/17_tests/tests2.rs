// tests2.rs - solution
//
// Returning `Result<(), String>` from a test lets you write `?` instead of
// `.unwrap()`. On failure the test reports the `Err` value. Use whatever
// error type the `?`s produce.
//
// `expected` matches a *substring* of the panic message. Checking it makes
// sure the test panicked for the reason you meant, not because of some
// unrelated bug.

fn parse_percent(text: &str) -> Result<u8, String> {
    let n: u8 = text
        .trim_end_matches('%')
        .parse()
        .map_err(|_| format!("{text:?} is not a percentage"))?;
    if n > 100 {
        return Err(format!("{n}% is more than 100%"));
    }
    Ok(n)
}

fn apply_discount(price: u32, percent: u8) -> u32 {
    if percent > 100 {
        panic!("discount of {percent}% would make the price negative");
    }
    price - price * percent as u32 / 100
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses() -> Result<(), String> {
        let p = parse_percent("25%")?;
        assert_eq!(p, 25);
        Ok(())
    }

    #[test]
    fn discount_from_text() -> Result<(), String> {
        let p = parse_percent("10%")?;
        assert_eq!(apply_discount(250, p), 225);
        Ok(())
    }

    #[test]
    #[should_panic(expected = "would make the price negative")]
    fn huge_discount_panics() {
        apply_discount(100, 150);
    }

    #[test]
    fn rejects_bad_input() {
        assert!(parse_percent("lots").is_err());
        assert_eq!(parse_percent("150%"), Err("150% is more than 100%".to_string()));
    }
}

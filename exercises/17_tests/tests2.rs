// tests2.rs
//
// Two more testing tools:
//
//   - A test can return `Result<(), E>`. Then it can use `?`, and it
//     fails if it returns an `Err`. The first two tests use `?` but are
//     missing their return types, and need to return `Ok(())` at the end.
//
//   - `#[should_panic(expected = "...")]` passes only if the test panics
//     with a message *containing* that text. The third test's expected
//     text is wrong. Run it, read the real panic message, and fix it.
//
// Don't change `parse_percent` or `apply_discount`.

// I AM NOT DONE

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
    fn parses() {
        let p = parse_percent("25%")?;
        assert_eq!(p, 25);
    }

    #[test]
    fn discount_from_text() {
        let p = parse_percent("10%")?;
        assert_eq!(apply_discount(250, p), 225);
    }

    #[test]
    #[should_panic(expected = "price can't be negative")]
    fn huge_discount_panics() {
        apply_discount(100, 150);
    }

    #[test]
    fn rejects_bad_input() {
        assert!(parse_percent("lots").is_err());
        assert_eq!(parse_percent("150%"), Err("150% is more than 100%".to_string()));
    }
}

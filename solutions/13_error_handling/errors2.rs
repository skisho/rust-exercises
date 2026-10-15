// errors2.rs - solution
//
// `expr?` means: if this is `Err(e)`, return `Err(e)` from the function
// right now; otherwise give me the value inside `Ok`. It only works in a
// function that itself returns a compatible `Result` (or Option).

use std::num::ParseIntError;

fn parse_size(text: &str) -> Result<(u32, u32), ParseIntError> {
    let (w, h) = text.split_once('x').unwrap_or((text, ""));
    let width = w.trim().parse::<u32>()?;
    let height = h.trim().parse::<u32>()?;
    Ok((width, height))
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid() {
        assert_eq!(parse_size("1920x1080"), Ok((1920, 1080)));
        assert_eq!(parse_size(" 800 x 600 "), Ok((800, 600)));
    }

    #[test]
    fn bad_width() {
        let err = parse_size("widex1080").unwrap_err();
        assert_eq!(err.to_string(), "invalid digit found in string");
    }

    #[test]
    fn missing_height() {
        let err = parse_size("1920").unwrap_err();
        assert_eq!(err.to_string(), "cannot parse integer from empty string");
    }
}

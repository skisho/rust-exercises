// errors4.rs - solution
//
// Any type implementing `std::error::Error` converts into
// `Box<dyn Error>` via `?`. Great for applications and quick scripts;
// libraries usually prefer a concrete error enum so callers can `match`.

use std::error::Error;

#[derive(Debug, PartialEq)]
struct Config {
    port: u16,
    verbose: bool,
}

fn load_config(port: &str, verbose: &str) -> Result<Config, Box<dyn Error>> {
    let port = port.parse::<u16>()?;
    let verbose = verbose.parse::<bool>()?;
    Ok(Config { port, verbose })
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid() {
        assert_eq!(
            load_config("8080", "true").unwrap(),
            Config { port: 8080, verbose: true }
        );
    }

    #[test]
    fn bad_port() {
        let err = load_config("http", "true").unwrap_err();
        assert_eq!(err.to_string(), "invalid digit found in string");
    }

    #[test]
    fn bad_flag() {
        let err = load_config("8080", "yes").unwrap_err();
        assert_eq!(err.to_string(), "provided string was not `true` or `false`");
    }
}

// lifetimes3.rs - solution
//
// `'static` means "valid for the entire program". Every string literal is
// a `&'static str`. Since `reason` only ever holds literals, saying so
// keeps `HttpStatus` free of lifetime parameters, so it can be stored
// anywhere, for as long as you like.
//
// Use `'static` when it's actually true. A struct that borrows from
// runtime data (like `Words` in lifetimes2) needs a real lifetime
// parameter instead.

#[derive(Debug, PartialEq)]
struct HttpStatus {
    code: u16,
    reason: &'static str,
}

fn status(code: u16) -> HttpStatus {
    let reason = match code {
        200 => "OK",
        404 => "Not Found",
        418 => "I'm a teapot",
        500 => "Internal Server Error",
        _ => "Unknown",
    };
    HttpStatus { code, reason }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_codes() {
        assert_eq!(status(404).reason, "Not Found");
        assert_eq!(status(418).reason, "I'm a teapot");
    }

    #[test]
    fn unknown_code() {
        assert_eq!(status(299), HttpStatus { code: 299, reason: "Unknown" });
    }

    #[test]
    fn keep_forever() {
        let mut log: Vec<HttpStatus> = Vec::new();
        for code in [200, 500] {
            log.push(status(code));
        }
        assert_eq!(log[1].reason, "Internal Server Error");
    }
}

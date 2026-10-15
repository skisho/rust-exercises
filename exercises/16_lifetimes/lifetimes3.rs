// lifetimes3.rs
//
// `status` builds an `HttpStatus` from a code. Its `reason` is always a
// string literal, which is baked into the program and lives for its whole
// run. Rust has a special lifetime for that.
//
// Make this compile. Don't add a lifetime *parameter* to `HttpStatus`.
// There's a simpler, more accurate annotation.

// I AM NOT DONE

#[derive(Debug, PartialEq)]
struct HttpStatus {
    code: u16,
    reason: &str,
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

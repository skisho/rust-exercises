// json.rs (challenge)
//
// Write a JSON serialiser. `Json` is a recursive enum describing any
// JSON value. Implement two ways to turn one into text:
//
//   1. `Display`: compact output with no whitespace at all:
//        {"name":"Ferris","tags":["crab",true]}
//
//   2. `pretty()`: two-space indentation, one item per line, and a
//      space after each colon. Empty arrays and objects stay "[]" / "{}".
//      See the `pretty` test for the exact format.
//
// Details that matter (read the JSON spec!):
//   - Strings *and object keys* need quotes and escaping: `"` -> `\"`,
//     `\` -> `\\`, newline -> `\n`, carriage return -> `\r`,
//     tab -> `\t`, and any other control character (below U+0020)
//     -> `\u00XX` with 4 hex digits.
//   - JSON has no NaN or infinity. Write those numbers as `null`.
//   - Rust already prints `9.0_f64` as "9", which is what we want.
//
// Tips: a helper `fn write_escaped(f: &mut fmt::Formatter, s: &str)` saves
// repeating yourself. For `pretty`, a recursive helper that takes the
// current indentation depth works well.
//
// Further reading:
//   - The JSON format: https://www.json.org/json-en.html
//   - std::fmt (format specifiers like {:04x}): https://doc.rust-lang.org/std/fmt/index.html
//   - Formatter: https://doc.rust-lang.org/std/fmt/struct.Formatter.html
//   - Implementing Display: https://doc.rust-lang.org/rust-by-example/hello/print/print_display.html

// I AM NOT DONE

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
enum Json {
    Null,
    Bool(bool),
    Number(f64),
    Str(String),
    Array(Vec<Json>),
    // A Vec rather than a HashMap, so keys keep their order.
    Object(Vec<(String, Json)>),
}

impl fmt::Display for Json {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}

impl Json {
    fn pretty(&self) -> String {
        todo!()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use Json::*;

    fn sample() -> Json {
        Object(vec![
            ("name".to_string(), Str("Ferris".to_string())),
            ("age".to_string(), Number(9.0)),
            (
                "tags".to_string(),
                Array(vec![Str("crab".to_string()), Bool(true), Null]),
            ),
            (
                "home".to_string(),
                Object(vec![("city".to_string(), Str("Rust".to_string()))]),
            ),
        ])
    }

    #[test]
    fn scalars() {
        assert_eq!(Null.to_string(), "null");
        assert_eq!(Bool(true).to_string(), "true");
        assert_eq!(Bool(false).to_string(), "false");
        assert_eq!(Number(42.0).to_string(), "42");
        assert_eq!(Number(-2.5).to_string(), "-2.5");
        assert_eq!(Str("hi".to_string()).to_string(), "\"hi\"");
    }

    #[test]
    fn non_finite_numbers_are_null() {
        assert_eq!(Number(f64::NAN).to_string(), "null");
        assert_eq!(Number(f64::INFINITY).to_string(), "null");
    }

    #[test]
    fn escaping() {
        let s = Str("say \"hi\"\\\n\ttab\u{1}".to_string());
        assert_eq!(s.to_string(), r#""say \"hi\"\\\n\ttab\u0001""#);
    }

    #[test]
    fn keys_are_escaped_too() {
        let obj = Object(vec![("a\"b".to_string(), Null)]);
        assert_eq!(obj.to_string(), r#"{"a\"b":null}"#);
    }

    #[test]
    fn compact() {
        assert_eq!(Array(vec![]).to_string(), "[]");
        assert_eq!(Object(vec![]).to_string(), "{}");
        assert_eq!(
            sample().to_string(),
            r#"{"name":"Ferris","age":9,"tags":["crab",true,null],"home":{"city":"Rust"}}"#
        );
    }

    #[test]
    fn pretty() {
        let expected = r#"{
  "name": "Ferris",
  "age": 9,
  "tags": [
    "crab",
    true,
    null
  ],
  "home": {
    "city": "Rust"
  }
}"#;
        assert_eq!(sample().pretty(), expected);
    }

    #[test]
    fn pretty_empty_and_scalar() {
        assert_eq!(Array(vec![]).pretty(), "[]");
        assert_eq!(
            Object(vec![("x".to_string(), Array(vec![]))]).pretty(),
            "{\n  \"x\": []\n}"
        );
        assert_eq!(Number(1.5).pretty(), "1.5");
    }
}

// json.rs (challenge) - solution
//
// `write!` into the Formatter, recursing with `{item}`, which calls
// `Display::fmt` on each child. `?` propagates write errors. `{:04x}`
// means "hex, zero-padded to width 4".
//
// `pretty` builds a String instead, so a helper can carry the
// indentation depth down through the recursion. Scalars and empty
// containers reuse the compact Display output.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
enum Json {
    Null,
    Bool(bool),
    Number(f64),
    Str(String),
    Array(Vec<Json>),
    Object(Vec<(String, Json)>),
}

fn write_escaped(f: &mut fmt::Formatter, s: &str) -> fmt::Result {
    write!(f, "\"")?;
    for c in s.chars() {
        match c {
            '"' => write!(f, "\\\"")?,
            '\\' => write!(f, "\\\\")?,
            '\n' => write!(f, "\\n")?,
            '\r' => write!(f, "\\r")?,
            '\t' => write!(f, "\\t")?,
            c if (c as u32) < 0x20 => write!(f, "\\u{:04x}", c as u32)?,
            c => write!(f, "{c}")?,
        }
    }
    write!(f, "\"")
}

/// Lets `write_escaped` be used with `{}` in format strings.
struct Escaped<'a>(&'a str);

impl fmt::Display for Escaped<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write_escaped(f, self.0)
    }
}

impl fmt::Display for Json {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Json::Null => write!(f, "null"),
            Json::Bool(b) => write!(f, "{b}"),
            Json::Number(n) if n.is_finite() => write!(f, "{n}"),
            Json::Number(_) => write!(f, "null"),
            Json::Str(s) => write_escaped(f, s),
            Json::Array(items) => {
                write!(f, "[")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{item}")?;
                }
                write!(f, "]")
            }
            Json::Object(fields) => {
                write!(f, "{{")?;
                for (i, (key, value)) in fields.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{}:{value}", Escaped(key))?;
                }
                write!(f, "}}")
            }
        }
    }
}

impl Json {
    fn pretty(&self) -> String {
        let mut out = String::new();
        self.pretty_into(&mut out, 0);
        out
    }

    fn pretty_into(&self, out: &mut String, depth: usize) {
        let indent = |out: &mut String, depth: usize| out.push_str(&"  ".repeat(depth));
        match self {
            Json::Array(items) if !items.is_empty() => {
                out.push_str("[\n");
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        out.push_str(",\n");
                    }
                    indent(out, depth + 1);
                    item.pretty_into(out, depth + 1);
                }
                out.push('\n');
                indent(out, depth);
                out.push(']');
            }
            Json::Object(fields) if !fields.is_empty() => {
                out.push_str("{\n");
                for (i, (key, value)) in fields.iter().enumerate() {
                    if i > 0 {
                        out.push_str(",\n");
                    }
                    indent(out, depth + 1);
                    out.push_str(&format!("{}: ", Escaped(key)));
                    value.pretty_into(out, depth + 1);
                }
                out.push('\n');
                indent(out, depth);
                out.push('}');
            }
            other => out.push_str(&other.to_string()),
        }
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
            ("tags".to_string(), Array(vec![Str("crab".to_string()), Bool(true), Null])),
            ("home".to_string(), Object(vec![("city".to_string(), Str("Rust".to_string()))])),
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
        assert_eq!(Object(vec![("x".to_string(), Array(vec![]))]).pretty(), "{\n  \"x\": []\n}");
        assert_eq!(Number(1.5).pretty(), "1.5");
    }
}

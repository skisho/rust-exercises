// errors1.rs
//
// `generate_nametag_text` returns `None` on bad input, so the caller
// can't tell *why* it failed. Change it to return a `Result` with an
// error message instead.

// I AM NOT DONE

fn generate_nametag_text(name: String) -> Option<String> {
    if name.is_empty() {
        None
    } else {
        Some(format!("Hi! My name is {name}"))
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".to_string()),
            Ok("Hi! My name is Beyoncé".to_string())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text(String::new()),
            Err("Empty names aren't allowed".to_string())
        );
    }
}

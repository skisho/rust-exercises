// errors1.rs - solution
//
// `Result<T, E>` is like `Option<T>`, but the failure case carries a
// value (`Err(E)`) explaining what went wrong.

fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        Err("Empty names aren't allowed".to_string())
    } else {
        Ok(format!("Hi! My name is {name}"))
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

// errors1.rs - solution
//
// `Result<T, E>` is like Option, but the failure case carries information.
// A `String` error is the simplest possible choice; later exercises use
// dedicated error types. The `&str` inside `Ok` borrows from `name`;
// lifetime elision ties the two together.

fn check_username(name: &str) -> Result<&str, String> {
    if name.is_empty() {
        Err("username can't be empty".to_string())
    } else if name.chars().count() > 12 {
        Err("username is too long".to_string())
    } else if !name.chars().all(|c| c.is_alphanumeric()) {
        Err("username may only contain letters and digits".to_string())
    } else {
        Ok(name)
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepted() {
        assert_eq!(check_username("ferris42"), Ok("ferris42"));
    }

    #[test]
    fn empty() {
        assert_eq!(check_username(""), Err("username can't be empty".to_string()));
    }

    #[test]
    fn too_long() {
        assert_eq!(
            check_username("averyverylongname"),
            Err("username is too long".to_string())
        );
    }

    #[test]
    fn bad_characters() {
        assert_eq!(
            check_username("ferris crab"),
            Err("username may only contain letters and digits".to_string())
        );
    }
}

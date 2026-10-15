// errors1.rs
//
// `check_username` returns a bool, so when a username is rejected, the
// user has no idea why. Change it to return a `Result`: `Ok` with the
// accepted username, or `Err` with a message explaining the problem.
//
// The rules, checked in this order:
//   - not empty                  "username can't be empty"
//   - at most 12 characters      "username is too long"
//   - only letters and digits    "username may only contain letters and digits"

// I AM NOT DONE

fn check_username(name: &str) -> bool {
    !name.is_empty()
        && name.chars().count() <= 12
        && name.chars().all(|c| c.is_alphanumeric())
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

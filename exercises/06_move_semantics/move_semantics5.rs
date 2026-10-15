// move_semantics5.rs
//
// `make_greeting` tries to return a reference to a String it created. But
// that String is dropped when the function returns, so the reference
// would point at freed memory. Rust calls this a "dangling reference"
// and refuses to compile it.
//
// Fix `make_greeting` so the caller gets something it can keep.

// I AM NOT DONE

fn make_greeting(name: &str) -> &str {
    let greeting = format!("Hello, {name}!");
    &greeting
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greets() {
        assert_eq!(make_greeting("Ferris"), "Hello, Ferris!");
    }

    #[test]
    fn greeting_outlives_the_name() {
        let greeting;
        {
            let name = String::from("Ada");
            greeting = make_greeting(&name);
        }
        assert_eq!(greeting, "Hello, Ada!");
    }
}

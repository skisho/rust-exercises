// move_semantics5.rs - solution
//
// When a function creates a new value, it should hand over ownership by
// returning the value itself, not a reference. Returning the `String`
// *moves* it to the caller, so nothing is freed and nothing is copied.
//
// A function can only return a reference if it points into one of its
// inputs (or into something `'static`).

fn make_greeting(name: &str) -> String {
    format!("Hello, {name}!")
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

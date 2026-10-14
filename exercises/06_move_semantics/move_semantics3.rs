// move_semantics3.rs
//
// `add_exclamation` should modify the caller's String without taking
// ownership of it. Fix the function signature and the call site.

// I AM NOT DONE

fn add_exclamation(s: String) {
    s.push('!');
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modifies_in_place() {
        let mut greeting = String::from("hello");
        add_exclamation(greeting);
        add_exclamation(greeting);
        assert_eq!(greeting, "hello!!");
    }
}

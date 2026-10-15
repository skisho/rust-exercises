// macros1.rs
//
// A `macro_rules!` macro is a list of rules: patterns on the left, the
// code to produce on the right. The first rule that matches is used.
//
// `greet!` has one rule, for no arguments. Add a second rule so that
// `greet!(name)` produces a greeting for that name. Watch out: macros
// must be defined *above* the code that uses them.

// I AM NOT DONE

fn main() {
    println!("{}", greet!());
    println!("{}", greet!("Ferris"));
}

macro_rules! greet {
    () => {
        String::from("Hello, world!")
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn no_arguments() {
        assert_eq!(greet!(), "Hello, world!");
    }

    #[test]
    fn with_a_name() {
        assert_eq!(greet!("Ferris"), "Hello, Ferris!");
        let crew = "Corro";
        assert_eq!(greet!(crew), "Hello, Corro!");
    }
}

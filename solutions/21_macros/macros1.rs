// macros1.rs - solution
//
// `$name:expr` matches any expression and binds it to `$name`. The
// expansion can use it like a variable. Macros are expanded in source
// order, so the definition has to come first in the file (or be imported
// with `#[macro_use]` or `use`).

macro_rules! greet {
    () => {
        String::from("Hello, world!")
    };
    ($name:expr) => {
        format!("Hello, {}!", $name)
    };
}

fn main() {
    println!("{}", greet!());
    println!("{}", greet!("Ferris"));
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

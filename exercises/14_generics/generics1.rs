// generics1.rs
//
// `Wrapper` only works with `u32`. Make it generic so it can hold a value
// of any type.

// I AM NOT DONE

struct Wrapper {
    value: u32,
}

impl Wrapper {
    fn new(value: u32) -> Self {
        Wrapper { value }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}

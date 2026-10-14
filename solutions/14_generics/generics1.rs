// generics1.rs - solution
//
// The type parameter `T` must be declared on the struct *and* on the
// `impl` block (`impl<T> Wrapper<T>`).

struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    fn new(value: T) -> Self {
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

// traits1.rs
//
// Implement the `AppendBar` trait for `String`. Appending "Bar" should
// return the string with "Bar" added to the end.

// I AM NOT DONE

trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    // TODO: implement `append_bar`
}

fn main() {
    let s = String::from("Foo");
    println!("s: {}", s.append_bar());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), "FooBar");
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(String::from("").append_bar().append_bar(), "BarBar");
    }
}

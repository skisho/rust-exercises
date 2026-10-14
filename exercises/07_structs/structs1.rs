// structs1.rs
//
// Define the `Color` struct and the `Point` tuple struct so the tests
// compile and pass.

// I AM NOT DONE

struct Color {
    // TODO: add fields
}

struct Point(/* TODO */);

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_struct() {
        let green = Color {
            red: 0,
            green: 255,
            blue: 0,
        };
        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_struct() {
        let p = Point(1.5, -2.0);
        assert_eq!(p.0, 1.5);
        assert_eq!(p.1, -2.0);
    }
}

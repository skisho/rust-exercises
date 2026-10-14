// structs1.rs - solution
//
// Classic structs have named fields; tuple structs have positional ones.

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

struct Point(f64, f64);

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_struct() {
        let green = Color { red: 0, green: 255, blue: 0 };
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

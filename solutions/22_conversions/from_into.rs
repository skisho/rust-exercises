// from_into.rs - solution
//
// Implement `From`, never `Into` directly: the standard library's blanket
// `impl<T, U: From<T>> Into<U> for T` provides `into()` automatically.
//
// `{:02x}` formats a number as lowercase hex, padded to two digits, so 0
// becomes "00" and 8 becomes "08".

#[derive(Debug, Clone, Copy, PartialEq)]
struct Celsius(f64);

#[derive(Debug, Clone, Copy, PartialEq)]
struct Fahrenheit(f64);

#[derive(Debug, Clone, Copy, PartialEq)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
    }
}

impl From<(u8, u8, u8)> for Rgb {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Rgb { r, g, b }
    }
}

impl From<Rgb> for String {
    fn from(c: Rgb) -> Self {
        format!("#{:02x}{:02x}{:02x}", c.r, c.g, c.b)
    }
}

fn main() {
    let body: Fahrenheit = Celsius(37.0).into();
    println!("{body:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn temperatures() {
        assert_eq!(Fahrenheit::from(Celsius(100.0)), Fahrenheit(212.0));
        let freezing: Fahrenheit = Celsius(0.0).into();
        assert_eq!(freezing, Fahrenheit(32.0));
        assert_eq!(Fahrenheit::from(Celsius(-40.0)), Fahrenheit(-40.0));
    }

    #[test]
    fn colour_from_tuple() {
        let orange: Rgb = (255, 128, 0).into();
        assert_eq!(orange, Rgb { r: 255, g: 128, b: 0 });
    }

    #[test]
    fn colour_to_hex() {
        assert_eq!(String::from(Rgb { r: 255, g: 128, b: 0 }), "#ff8000");
        let teal: String = Rgb::from((0, 128, 128)).into();
        assert_eq!(teal, "#008080");
    }
}

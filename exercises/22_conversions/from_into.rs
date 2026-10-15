// from_into.rs
//
// `From` describes an infallible conversion. Implement:
//   - `From<Celsius> for Fahrenheit`   (F = C * 9 / 5 + 32)
//   - `From<(u8, u8, u8)> for Rgb`
//   - `From<Rgb> for String`, giving lowercase hex like "#ff8000"
//
// Once `From<A> for B` exists, you get `Into<B> for A` for free, so
// `a.into()` works too. The tests use both.

// I AM NOT DONE

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

// TODO: the three `From` implementations

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

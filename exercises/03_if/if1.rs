// if1.rs
//
// Implement `describe_temperature` with an `if` / `else if` / `else`
// chain. Temperatures are in °C:
//   - 0 or below:          "freezing"
//   - above 0, below 15:   "cold"
//   - 15 up to below 25:   "mild"
//   - 25 or above:         "hot"

// I AM NOT DONE

fn describe_temperature(celsius: f64) -> &'static str {
    // TODO
}

fn main() {
    println!("18.5°C is {}", describe_temperature(18.5));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn freezing() {
        assert_eq!(describe_temperature(-12.0), "freezing");
        assert_eq!(describe_temperature(0.0), "freezing");
    }

    #[test]
    fn cold() {
        assert_eq!(describe_temperature(0.1), "cold");
        assert_eq!(describe_temperature(14.9), "cold");
    }

    #[test]
    fn mild() {
        assert_eq!(describe_temperature(15.0), "mild");
        assert_eq!(describe_temperature(24.99), "mild");
    }

    #[test]
    fn hot() {
        assert_eq!(describe_temperature(25.0), "hot");
        assert_eq!(describe_temperature(41.0), "hot");
    }
}

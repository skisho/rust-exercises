// if1.rs - solution
//
// The branches are checked top to bottom and the first true one wins, so
// each condition only needs to state its upper bound. The whole `if` is
// an expression, and every branch must have the same type (`&str` here).

fn describe_temperature(celsius: f64) -> &'static str {
    if celsius <= 0.0 {
        "freezing"
    } else if celsius < 15.0 {
        "cold"
    } else if celsius < 25.0 {
        "mild"
    } else {
        "hot"
    }
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

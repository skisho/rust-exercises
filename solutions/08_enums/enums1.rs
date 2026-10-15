// enums1.rs - solution
//
// A `match` on an enum must cover every variant. If you later add, say,
// `FlashingYellow`, the compiler points at every `match` that needs
// updating. That's a big part of why enums are so useful in Rust.

#[derive(Debug, Clone, Copy, PartialEq)]
enum Light {
    Green,
    Yellow,
    Red,
}

impl Light {
    fn next(self) -> Light {
        match self {
            Light::Green => Light::Yellow,
            Light::Yellow => Light::Red,
            Light::Red => Light::Green,
        }
    }

    fn seconds(self) -> u32 {
        match self {
            Light::Green => 30,
            Light::Yellow => 3,
            Light::Red => 20,
        }
    }
}

fn main() {
    let mut light = Light::Green;
    for _ in 0..4 {
        println!("{light:?} for {}s", light.seconds());
        light = light.next();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cycle() {
        assert_eq!(Light::Green.next(), Light::Yellow);
        assert_eq!(Light::Yellow.next(), Light::Red);
        assert_eq!(Light::Red.next(), Light::Green);
    }

    #[test]
    fn full_cycle_takes_53_seconds() {
        let mut light = Light::Red;
        let mut total = 0;
        for _ in 0..3 {
            total += light.seconds();
            light = light.next();
        }
        assert_eq!(total, 53);
        assert_eq!(light, Light::Red);
    }
}

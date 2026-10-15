// enums1.rs
//
// A traffic light cycles Green -> Yellow -> Red -> Green ...
//
// 1. Add the three variants to `Light`.
// 2. Implement `next` and `seconds` with `match`. Green lasts 30 seconds,
//    Yellow 3, and Red 20.

// I AM NOT DONE

#[derive(Debug, Clone, Copy, PartialEq)]
enum Light {
    // TODO
}

impl Light {
    fn next(self) -> Light {
        todo!()
    }

    fn seconds(self) -> u32 {
        todo!()
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

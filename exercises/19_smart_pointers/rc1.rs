// rc1.rs
//
// Several planets share the same sun. `Rc` (reference counted) lets
// multiple owners share one value. Replace the TODOs so the reference
// count assertions pass.

// I AM NOT DONE

use std::rc::Rc;

#[derive(Debug)]
struct Sun;

#[derive(Debug)]
struct Planet {
    name: &'static str,
    sun: Rc<Sun>,
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shared_sun() {
        let sun = Rc::new(Sun);
        assert_eq!(Rc::strong_count(&sun), 1);

        let mercury = Planet {
            name: "Mercury",
            sun: Rc::clone(&sun),
        };
        assert_eq!(Rc::strong_count(&sun), 2);

        // TODO: create `venus` and `earth`, both sharing `sun`.
        let venus = Planet {
            name: "Venus",
            sun: todo!(),
        };
        let earth = Planet {
            name: "Earth",
            sun: todo!(),
        };
        assert_eq!(Rc::strong_count(&sun), 4);
        println!("{} {} {}", mercury.name, venus.name, earth.name);

        // TODO: get rid of `earth` so the count drops back down.
        assert_eq!(Rc::strong_count(&sun), 3);

        drop(mercury);
        drop(venus);
        assert_eq!(Rc::strong_count(&sun), 1);
    }
}

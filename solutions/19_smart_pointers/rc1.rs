// rc1.rs - solution
//
// `Rc::clone` doesn't copy the Sun: it bumps a counter and hands out
// another pointer to the same value. Dropping an owner decrements it.

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

        let mercury = Planet { name: "Mercury", sun: Rc::clone(&sun) };
        assert_eq!(Rc::strong_count(&sun), 2);

        let venus = Planet { name: "Venus", sun: Rc::clone(&sun) };
        let earth = Planet { name: "Earth", sun: Rc::clone(&sun) };
        assert_eq!(Rc::strong_count(&sun), 4);
        println!("{} {} {}", mercury.name, venus.name, earth.name);

        drop(earth);
        assert_eq!(Rc::strong_count(&sun), 3);

        drop(mercury);
        drop(venus);
        assert_eq!(Rc::strong_count(&sun), 1);
    }
}

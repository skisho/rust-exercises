// from_into.rs - solution
//
// Implementing `From<A> for B` gives you `Into<B> for A` for free.
// `split_once` neatly rejects input with no comma; a second comma ends
// up in `age_str`, which then fails to parse.

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: u8,
}

impl Default for Person {
    fn default() -> Self {
        Person { name: String::from("John"), age: 30 }
    }
}

impl From<&str> for Person {
    fn from(s: &str) -> Self {
        let Some((name, age_str)) = s.split_once(',') else {
            return Person::default();
        };
        if name.is_empty() {
            return Person::default();
        }
        match age_str.parse::<u8>() {
            Ok(age) => Person { name: name.to_string(), age },
            Err(_) => Person::default(),
        }
    }
}

fn main() {
    let p: Person = "Ferris,9".into();
    println!("{p:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn good_input() {
        let p = Person::from("Mark,20");
        assert_eq!(p, Person { name: "Mark".to_string(), age: 20 });
    }

    #[test]
    fn into_works_for_free() {
        let p: Person = "Ada,36".into();
        assert_eq!(p.name, "Ada");
    }

    #[test]
    fn bad_inputs_give_default() {
        for bad in ["", "Mark", ",20", "Mark,", "Mark,twenty", "Mark,20,extra"] {
            assert_eq!(Person::from(bad), Person::default(), "input: {bad:?}");
        }
    }
}

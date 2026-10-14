// from_into.rs
//
// Implement `From<&str>` for `Person`. The input looks like "Mark,20".
// If anything about the input is wrong (missing comma, empty name,
// unparseable age, extra fields), return `Person::default()` instead.

// I AM NOT DONE

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: u8,
}

impl Default for Person {
    fn default() -> Self {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}

impl From<&str> for Person {
    fn from(s: &str) -> Self {
        todo!()
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
        assert_eq!(
            p,
            Person {
                name: "Mark".to_string(),
                age: 20
            }
        );
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

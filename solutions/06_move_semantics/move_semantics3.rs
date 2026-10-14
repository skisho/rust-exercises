// move_semantics3.rs - solution
//
// A mutable reference (`&mut String`) lets a function change a value it
// doesn't own. The caller lends it with `&mut greeting`.

fn add_exclamation(s: &mut String) {
    s.push('!');
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modifies_in_place() {
        let mut greeting = String::from("hello");
        add_exclamation(&mut greeting);
        add_exclamation(&mut greeting);
        assert_eq!(greeting, "hello!!");
    }
}

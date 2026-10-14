// primitive_types2.rs
//
// Tuples group values of different types together.
// Make the tests pass by filling in the TODOs.

// I AM NOT DONE

fn main() {}

#[cfg(test)]
mod tests {
    #[test]
    fn destructure() {
        let cat = ("Furry McFurson", 3.5);
        // TODO: destructure `cat` into `name` and `age` in a single `let`.
        ???
        assert_eq!(name, "Furry McFurson");
        assert_eq!(age, 3.5);
    }

    #[test]
    fn index_into_tuple() {
        let numbers = (1, 2, 3);
        // TODO: use tuple indexing to get the second element.
        let second = ???;
        assert_eq!(second, 2);
    }
}

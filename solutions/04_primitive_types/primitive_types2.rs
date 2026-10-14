// primitive_types2.rs - solution
//
// A pattern on the left of `let` can take a tuple apart. Individual
// elements can also be read with `.0`, `.1`, ...

fn main() {}

#[cfg(test)]
mod tests {
    #[test]
    fn destructure() {
        let cat = ("Furry McFurson", 3.5);
        let (name, age) = cat;
        assert_eq!(name, "Furry McFurson");
        assert_eq!(age, 3.5);
    }

    #[test]
    fn index_into_tuple() {
        let numbers = (1, 2, 3);
        let second = numbers.1;
        assert_eq!(second, 2);
    }
}

// structs1.rs
//
// 1. Define the `Book` struct so the tests compile. Look at how the tests
//    use it to work out the field names and types.
// 2. Implement `new_book` using the *field init shorthand*: when a
//    variable has the same name as a field, you can write it just once.
// 3. Implement `second_edition`: a copy of the book with `edition` set
//    to 2 and every other field the same. Use *struct update syntax*
//    (`..`) so you don't have to list the other fields.

// I AM NOT DONE

#[derive(Debug, Clone, PartialEq)]
struct Book {
    // TODO
}

fn new_book(title: &str, author: &str, pages: u32) -> Book {
    let title = title.to_string();
    let author = author.to_string();
    todo!()
}

fn second_edition(book: &Book) -> Book {
    todo!()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let b = new_book("Dune", "Frank Herbert", 412);
        assert_eq!(b.title, "Dune");
        assert_eq!(b.author, "Frank Herbert");
        assert_eq!(b.pages, 412);
        assert_eq!(b.edition, 1);
    }

    #[test]
    fn update() {
        let first = new_book("Dune", "Frank Herbert", 412);
        let second = second_edition(&first);
        assert_eq!(second.edition, 2);
        assert_eq!(second.title, first.title);
        assert_eq!(second.pages, 412);
        assert_eq!(first.edition, 1, "the first edition must be unchanged");
    }
}

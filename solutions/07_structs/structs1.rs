// structs1.rs - solution
//
// `Book { title, author, ... }` is shorthand for `title: title, ...`.
//
// `..other` fills every field you didn't mention from `other`. It *moves*
// those fields, so we spread a clone: spreading `*book` directly would try
// to move the Strings out of a borrowed value.

#[derive(Debug, Clone, PartialEq)]
struct Book {
    title: String,
    author: String,
    pages: u32,
    edition: u32,
}

fn new_book(title: &str, author: &str, pages: u32) -> Book {
    let title = title.to_string();
    let author = author.to_string();
    Book {
        title,
        author,
        pages,
        edition: 1,
    }
}

fn second_edition(book: &Book) -> Book {
    Book {
        edition: 2,
        ..book.clone()
    }
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

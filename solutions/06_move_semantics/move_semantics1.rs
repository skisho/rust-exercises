// move_semantics1.rs - solution
//
// Passing a `Vec<String>` by value moves it into the function, which drops
// it at the end. Taking a borrowed slice, `&[String]`, lets the caller
// keep ownership. `&names` converts automatically from `&Vec<String>` to
// `&[String]`. A slice parameter is preferred over `&Vec<String>`, because
// it also accepts arrays and parts of other Vecs.

fn total_letters(words: &[String]) -> usize {
    let mut total = 0;
    for word in words.iter() {
        total += word.len();
    }
    total
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_twice() {
        let mut names = vec![String::from("Ada"), String::from("Grace")];
        assert_eq!(total_letters(&names), 8);

        names.push(String::from("Alan"));
        assert_eq!(total_letters(&names), 12);
    }
}

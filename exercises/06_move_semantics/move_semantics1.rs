// move_semantics1.rs
//
// Make the test compile. `total_letters` takes ownership of the Vec, so
// after the first call, `names` is gone and can't be used again.
//
// Fix it by changing `total_letters` so it only *borrows* the names, and
// update the calls to match. Don't use `.clone()`.

// I AM NOT DONE

fn total_letters(words: Vec<String>) -> usize {
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
        assert_eq!(total_letters(names), 8);

        names.push(String::from("Alan"));
        assert_eq!(total_letters(names), 12);
    }
}

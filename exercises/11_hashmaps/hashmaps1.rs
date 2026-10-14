// hashmaps1.rs
//
// Build a fruit basket with at least three kinds of fruit and at least
// five pieces of fruit in total.

// I AM NOT DONE

use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    let mut basket = // TODO: create an empty HashMap

    basket.insert(String::from("banana"), 2);

    // TODO: put more fruit in the basket

    basket
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_kinds() {
        assert!(fruit_basket().len() >= 3);
    }

    #[test]
    fn at_least_five_pieces() {
        assert!(fruit_basket().values().sum::<u32>() >= 5);
    }
}

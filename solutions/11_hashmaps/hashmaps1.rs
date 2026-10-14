// hashmaps1.rs - solution
//
// `HashMap::new()` creates an empty map; the key and value types are
// inferred from the function's return type.

use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    let mut basket = HashMap::new();

    basket.insert(String::from("banana"), 2);
    basket.insert(String::from("apple"), 3);
    basket.insert(String::from("mango"), 1);

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

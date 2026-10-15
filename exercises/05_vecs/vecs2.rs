// vecs2.rs
//
// A shop keeps stock levels in a Vec of (item name, quantity) pairs.
//   - `restock` adds `amount` to every quantity, modifying the Vec in place
//   - `remove_sold_out` drops every item whose quantity is 0
//   - `quantity_at` returns the quantity at a position, or None if the
//     position is past the end. It must never panic!

// I AM NOT DONE

fn restock(stock: &mut Vec<(String, u32)>, amount: u32) {
    for item in stock.iter_mut() {
        // TODO: `item` is a `&mut (String, u32)`. Add to its quantity.
    }
}

fn remove_sold_out(stock: &mut Vec<(String, u32)>) {
    // TODO: look at `Vec::retain`.
}

fn quantity_at(stock: &[(String, u32)], index: usize) -> Option<u32> {
    // TODO: `stock[index]` panics when out of bounds. What doesn't?
    Some(stock[index].1)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    fn shop() -> Vec<(String, u32)> {
        vec![
            ("apples".to_string(), 3),
            ("bread".to_string(), 0),
            ("cheese".to_string(), 5),
            ("dates".to_string(), 0),
        ]
    }

    #[test]
    fn restocking() {
        let mut stock = shop();
        restock(&mut stock, 10);
        let quantities: Vec<u32> = stock.iter().map(|(_, q)| *q).collect();
        assert_eq!(quantities, [13, 10, 15, 10]);
    }

    #[test]
    fn clearing_out() {
        let mut stock = shop();
        remove_sold_out(&mut stock);
        let names: Vec<&str> = stock.iter().map(|(n, _)| n.as_str()).collect();
        assert_eq!(names, ["apples", "cheese"]);
    }

    #[test]
    fn safe_lookup() {
        let stock = shop();
        assert_eq!(quantity_at(&stock, 2), Some(5));
        assert_eq!(quantity_at(&stock, 4), None);
        assert_eq!(quantity_at(&[], 0), None);
    }
}

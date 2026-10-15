// vecs2.rs - solution
//
// Through a `&mut (String, u32)` you can reach the fields directly:
// `item.1 += amount` auto-dereferences. `retain` keeps only the elements
// for which the closure returns true. `get` is the non-panicking version
// of indexing and returns an Option.

fn restock(stock: &mut Vec<(String, u32)>, amount: u32) {
    for item in stock.iter_mut() {
        item.1 += amount;
    }
}

fn remove_sold_out(stock: &mut Vec<(String, u32)>) {
    stock.retain(|(_, quantity)| *quantity > 0);
}

fn quantity_at(stock: &[(String, u32)], index: usize) -> Option<u32> {
    stock.get(index).map(|(_, quantity)| *quantity)
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

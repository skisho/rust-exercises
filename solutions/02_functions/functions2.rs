// functions2.rs - solution
//
// `return` exits the function immediately and is handy for guard clauses
// like the free case. Everywhere else, Rust style is to let the final
// expression (no semicolon!) be the return value. Integer division
// already rounds down.

fn ticket_price(age: u32, is_tuesday: bool) -> u32 {
    if age < 3 {
        return 0;
    }

    let base = if age <= 12 {
        6
    } else if age >= 65 {
        7
    } else {
        10
    };

    if is_tuesday { base / 2 } else { base }
}

fn main() {
    println!("an adult on a Tuesday pays {}", ticket_price(30, true));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toddlers_are_free() {
        assert_eq!(ticket_price(0, false), 0);
        assert_eq!(ticket_price(2, true), 0);
    }

    #[test]
    fn regular_days() {
        assert_eq!(ticket_price(3, false), 6);
        assert_eq!(ticket_price(12, false), 6);
        assert_eq!(ticket_price(13, false), 10);
        assert_eq!(ticket_price(64, false), 10);
        assert_eq!(ticket_price(65, false), 7);
    }

    #[test]
    fn tuesdays() {
        assert_eq!(ticket_price(8, true), 3);
        assert_eq!(ticket_price(40, true), 5);
        assert_eq!(ticket_price(70, true), 3);
    }
}

// functions2.rs
//
// A cinema prices its tickets like this:
//   - under 3:      free (0)
//   - 3 to 12:      6
//   - 65 and over:  7
//   - everyone else 10
//   - and on Tuesdays, every non-free ticket costs half (round down)
//
// Implement `ticket_price`. Use `return` to leave early for the free case,
// then let the *last expression* of the function be the result for the
// rest.

// I AM NOT DONE

fn ticket_price(age: u32, is_tuesday: bool) -> u32 {
    // TODO
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

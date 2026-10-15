// primitive_types1.rs
//
// `char` is a single Unicode character (written with single quotes),
// `bool` is true/false, and `as` converts between number-like types.
// Make the tests pass by implementing the functions and replacing `???`.

// I AM NOT DONE

fn is_vowel(c: char) -> bool {
    // TODO: true for a, e, i, o, u in either case.
    // Look at `to_ascii_lowercase` and the `matches!` macro.
    todo!()
}

fn digit_value(c: char) -> u32 {
    // TODO: '0' -> 0, '7' -> 7, ... Characters are numbered in order,
    // so subtracting the number of '0' works. Use `as u32` on both.
    todo!()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vowels() {
        assert!(is_vowel('a'));
        assert!(is_vowel('U'));
        assert!(!is_vowel('z'));
        assert!(!is_vowel('?'));
    }

    #[test]
    fn digits() {
        assert_eq!(digit_value('0'), 0);
        assert_eq!(digit_value('7'), 7);
    }

    #[test]
    fn bytes_versus_chars() {
        // `len()` counts *bytes*, not characters. Accented letters and
        // emoji take more than one byte in UTF-8.
        let word = "café";
        assert_eq!(word.len(), ???);
        assert_eq!(word.chars().count(), ???);
    }

    #[test]
    fn casting_truncates() {
        // `as` from a float to an integer chops off the fraction, and
        // `as` to a smaller integer type keeps only the low bits.
        assert_eq!(9.99_f64 as i32, ???);
        assert_eq!(300_i32 as u8, ???);
    }
}

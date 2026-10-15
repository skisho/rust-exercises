// primitive_types1.rs - solution
//
// "café" is 5 bytes: 'é' needs two bytes in UTF-8. That's why Rust won't
// let you index a string by position: byte 4 isn't a whole character.
//
// `as` never fails, so it silently loses information: 9.99 becomes 9, and
// 300 wraps to 300 - 256 = 44 in a u8. Later you'll meet `try_from` for
// conversions that should report a problem instead.

fn is_vowel(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
}

fn digit_value(c: char) -> u32 {
    c as u32 - '0' as u32
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
        let word = "café";
        assert_eq!(word.len(), 5);
        assert_eq!(word.chars().count(), 4);
    }

    #[test]
    fn casting_truncates() {
        assert_eq!(9.99_f64 as i32, 9);
        assert_eq!(300_i32 as u8, 44);
    }
}

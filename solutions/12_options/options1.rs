// options1.rs - solution
//
// To use the value inside an Option you must handle the None case. In a
// test, `unwrap()` (panic on None) is fine; in real code prefer `match`,
// `if let`, `unwrap_or`, or `?`.

fn first_even(numbers: &[i32]) -> Option<i32> {
    for &n in numbers {
        if n % 2 == 0 {
            return Some(n);
        }
    }
    None
    // Or, using iterators: numbers.iter().copied().find(|n| n % 2 == 0)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_even() {
        assert_eq!(first_even(&[1, 3, 4, 6]), Some(4));
    }

    #[test]
    fn no_even() {
        assert_eq!(first_even(&[1, 3, 5]), None);
    }

    #[test]
    fn use_the_value() {
        let doubled: i32 = first_even(&[7, 10]).unwrap() * 2;
        assert_eq!(doubled, 20);
    }
}

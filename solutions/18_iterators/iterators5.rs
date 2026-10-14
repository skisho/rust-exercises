// iterators5.rs - solution
//
// `enumerate` pairs items with their index, `windows(n)` yields
// overlapping slices, `zip` walks two iterators in lockstep, and
// `take_while` stops at the first failing item. `collect` can build any
// collection implementing `FromIterator`, including a HashSet.

use std::collections::HashSet;

fn numbered(items: &[&str]) -> Vec<String> {
    items
        .iter()
        .enumerate()
        .map(|(i, item)| format!("{}. {item}", i + 1))
        .collect()
}

fn differences(values: &[i32]) -> Vec<i32> {
    values.windows(2).map(|pair| pair[1] - pair[0]).collect()
}

fn dot_product(a: &[i32], b: &[i32]) -> i32 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}

fn leading_non_negative(values: &[i32]) -> Vec<i32> {
    values.iter().copied().take_while(|&n| n >= 0).collect()
}

fn count_distinct(words: &[&str]) -> usize {
    words.iter().collect::<HashSet<_>>().len()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbering() {
        assert_eq!(numbered(&["apple", "pear"]), ["1. apple", "2. pear"]);
    }

    #[test]
    fn neighbour_differences() {
        assert_eq!(differences(&[1, 4, 9, 16]), [3, 5, 7]);
        assert!(differences(&[42]).is_empty());
    }

    #[test]
    fn dot() {
        assert_eq!(dot_product(&[1, 2, 3], &[4, 5, 6]), 32);
    }

    #[test]
    fn leading() {
        assert_eq!(leading_non_negative(&[3, 1, 4, -1, 5]), [3, 1, 4]);
        assert!(leading_non_negative(&[-2, 3]).is_empty());
    }

    #[test]
    fn distinct() {
        assert_eq!(count_distinct(&["a", "bb", "a", "ccc"]), 3);
    }
}

// iterators5.rs
//
// A tour of some more iterator adapters. Implement each function with a
// single iterator chain. The adapter to use is named in each comment.

// I AM NOT DONE

// ["apple", "pear"] -> ["1. apple", "2. pear"]          (enumerate)
fn numbered(items: &[&str]) -> Vec<String> {
    todo!()
}

// [1, 4, 9, 16] -> [3, 5, 7]: differences between neighbours  (windows)
fn differences(values: &[i32]) -> Vec<i32> {
    todo!()
}

// [1, 2, 3] . [4, 5, 6] = 1*4 + 2*5 + 3*6 = 32           (zip)
fn dot_product(a: &[i32], b: &[i32]) -> i32 {
    todo!()
}

// [3, 1, 4, -1, 5] -> [3, 1, 4]: stop at the first negative  (take_while)
fn leading_non_negative(values: &[i32]) -> Vec<i32> {
    todo!()
}

// ["a", "bb", "a", "ccc"] -> 3 distinct values  (collect into a HashSet)
fn count_distinct(words: &[&str]) -> usize {
    todo!()
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

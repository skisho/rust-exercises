// vecs2.rs - solution
//
// `iter_mut()` yields `&mut i32`; dereference with `*` to change the value
// behind the reference. `map` + `collect` builds a new collection.

fn double_in_place(v: &mut Vec<i32>) {
    for element in v.iter_mut() {
        *element *= 2;
    }
}

fn doubled(v: &[i32]) -> Vec<i32> {
    v.iter().map(|n| n * 2).collect()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_place() {
        let mut v = vec![1, 2, 3, 4];
        double_in_place(&mut v);
        assert_eq!(v, [2, 4, 6, 8]);
    }

    #[test]
    fn new_vec() {
        let v = vec![1, 2, 3, 4];
        assert_eq!(doubled(&v), [2, 4, 6, 8]);
        assert_eq!(v, [1, 2, 3, 4], "the original must be unchanged");
    }
}

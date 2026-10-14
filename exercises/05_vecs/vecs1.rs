// vecs1.rs
//
// A `Vec` is a growable array. Make `v` a Vec containing the same
// elements as `a`, using the `vec!` macro.

// I AM NOT DONE

fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40];
    let v = // TODO

    (a, v)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_contents() {
        let (a, v) = array_and_vec();
        assert_eq!(a, v[..]);
    }
}

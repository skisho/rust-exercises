// vecs1.rs - solution
//
// `vec![...]` builds a Vec with the given elements. `a.to_vec()` would
// also work.

fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40];
    let v = vec![10, 20, 30, 40];

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

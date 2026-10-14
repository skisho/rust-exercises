// move_semantics2.rs - solution
//
// Passing `v` by value moves it into `fill_vec`, so it can't be used
// afterwards. Passing a clone keeps the original around. (Another option
// is to change `fill_vec` to take `&[i32]` and build a new Vec inside.)

fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(88);
    vec
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn original_is_kept() {
        let v = vec![22, 44, 66];
        let filled = fill_vec(v.clone());

        assert_eq!(v, [22, 44, 66]);
        assert_eq!(filled, [22, 44, 66, 88]);
    }
}

// move_semantics2.rs
//
// Make the test compile and pass. You may change the test this time, but
// keep both assertions.

// I AM NOT DONE

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
        let filled = fill_vec(v);

        assert_eq!(v, [22, 44, 66]);
        assert_eq!(filled, [22, 44, 66, 88]);
    }
}

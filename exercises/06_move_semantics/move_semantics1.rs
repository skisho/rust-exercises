// move_semantics1.rs
//
// Make this compile. Only change the `fill_vec` function.

// I AM NOT DONE

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    vec.push(88);
    vec
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fills() {
        let v = vec![22, 44, 66];
        let filled = fill_vec(v);
        assert_eq!(filled, [22, 44, 66, 88]);
    }
}

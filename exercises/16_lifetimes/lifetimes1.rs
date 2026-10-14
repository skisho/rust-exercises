// lifetimes1.rs
//
// The compiler can't tell whether the returned reference borrows from
// `x` or from `y`. Add lifetime annotations so this compiles.

// I AM NOT DONE

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picks_longest() {
        assert_eq!(longest("abcd", "xyz"), "abcd");
        assert_eq!(longest("a", "xyz"), "xyz");
    }
}

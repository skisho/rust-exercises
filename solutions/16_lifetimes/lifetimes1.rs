// lifetimes1.rs - solution
//
// `'a` says: the returned reference is valid for as long as *both*
// inputs are valid. Lifetimes describe relationships; they don't change
// how long anything actually lives.

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
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

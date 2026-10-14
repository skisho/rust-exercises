// functions3.rs - solution
//
// 1. A function that returns a value must declare its return type: `-> i32`.
// 2. The last expression in a block is its value, but only if it has no
//    trailing semicolon. `n * n;` is a statement that evaluates to `()`.

fn square(n: i32) -> i32 {
    n * n
}

fn main() {
    println!("4 squared is {}", square(4));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn squares() {
        assert_eq!(square(3), 9);
        assert_eq!(square(-5), 25);
    }
}

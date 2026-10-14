// functions3.rs
//
// Make the tests pass. This file has two separate problems.

// I AM NOT DONE

fn square(n: i32) {
    n * n;
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

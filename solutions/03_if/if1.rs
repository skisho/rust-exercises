// if1.rs - solution
//
// `if` is an expression in Rust, so its value can be returned directly.

fn bigger(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

fn main() {
    println!("the bigger of 3 and 7 is {}", bigger(3, 7));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(bigger(10, 8), 10);
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(bigger(32, 42), 42);
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(bigger(5, 5), 5);
    }
}

// lifetimes3.rs
//
// `longest` is correct. The problem is in the test: `result` might
// borrow from `string2`, which is dropped at the end of the inner block,
// while `result` is still used after it.
//
// Fix the test without changing `longest` and without cloning anything.
// Keep the assertion.

// I AM NOT DONE

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result_outlives_inputs() {
        let string1 = String::from("long string is long");
        let result;
        {
            let string2 = String::from("xyz");
            result = longest(&string1, &string2);
        }
        assert_eq!(result, "long string is long");
    }
}

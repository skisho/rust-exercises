// lifetimes3.rs - solution
//
// We know `result` points into `string1`, but the compiler only knows
// what the signature says: the output lives as long as the *shorter* of
// the two inputs. So every input must outlive every use of `result`.
// Here `string2` moves to the outer scope. (Moving the assertion into
// the inner block would also work.)

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
        let string2 = String::from("xyz");
        let result = longest(&string1, &string2);
        assert_eq!(result, "long string is long");
    }
}

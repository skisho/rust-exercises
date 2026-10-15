// functions3.rs - solution
//
// `fn(i32) -> i32` is a function pointer type. Any function with a
// matching signature can be passed by name, with no parentheses. (Later
// you'll meet closures, which are accepted with `impl Fn(i32) -> i32`.)

fn add_three(n: i32) -> i32 {
    n + 3
}

fn double(n: i32) -> i32 {
    n * 2
}

fn negate(n: i32) -> i32 {
    -n
}

fn apply_twice(f: fn(i32) -> i32, start: i32) -> i32 {
    f(f(start))
}

fn main() {
    println!("{}", apply_twice(double, 5));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn twice() {
        assert_eq!(apply_twice(add_three, 1), 7);
        assert_eq!(apply_twice(double, 5), 20);
    }

    #[test]
    fn negating_twice_changes_nothing() {
        assert_eq!(apply_twice(negate, 42), 42);
        assert_eq!(negate(9), -9);
    }
}

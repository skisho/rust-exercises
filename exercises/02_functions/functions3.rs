// functions3.rs
//
// Functions are values too: you can pass one to another function. The
// type of "a function taking an i32 and returning an i32" is written
// `fn(i32) -> i32`.
//
// 1. Give `apply_twice` a signature: it takes such a function and a
//    starting value, and returns an i32.
// 2. Write the missing `negate` function so the last test passes.

// I AM NOT DONE

fn add_three(n: i32) -> i32 {
    n + 3
}

fn double(n: i32) -> i32 {
    n * 2
}

fn apply_twice(/* TODO */) {
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

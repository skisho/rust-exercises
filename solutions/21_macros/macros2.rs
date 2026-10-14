// macros2.rs - solution
//
// `$x:expr` matches an expression. `$($rest:expr),+` matches one or more
// comma-separated expressions, which can be passed on with the same
// repetition syntax. Rules are tried top to bottom.

macro_rules! max {
    ($x:expr) => {
        $x
    };
    ($x:expr, $($rest:expr),+) => {
        std::cmp::max($x, max!($($rest),+))
    };
}

fn main() {}

#[cfg(test)]
mod tests {
    #[test]
    fn single() {
        assert_eq!(max!(7), 7);
    }

    #[test]
    fn several() {
        assert_eq!(max!(3, 9, 4), 9);
        assert_eq!(max!(1, 2, 3, 4, 5, 2), 5);
    }

    #[test]
    fn expressions() {
        assert_eq!(max!(2 * 5, 3 + 3), 10);
    }
}

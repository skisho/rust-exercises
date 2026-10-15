// box1.rs
//
// `Expr` is a tiny arithmetic expression tree: `(2 + 3) * 4` is a `Mul`
// whose left side is an `Add`. But an enum that contains itself directly
// would be infinitely large, so this doesn't compile.
//
// 1. Fix `Expr` using `Box`, which stores a value on the heap and has a
//    fixed size itself.
// 2. Implement `eval`.
// 3. Build the expression in `example` (the helper functions make it
//    easier to read).

// I AM NOT DONE

#[derive(Debug)]
enum Expr {
    Num(i64),
    Add(Expr, Expr),
    Mul(Expr, Expr),
    Neg(Expr),
}

use Expr::{Add, Mul, Neg, Num};

fn add(a: Expr, b: Expr) -> Expr {
    Add(Box::new(a), Box::new(b))
}

fn mul(a: Expr, b: Expr) -> Expr {
    Mul(Box::new(a), Box::new(b))
}

fn neg(a: Expr) -> Expr {
    Neg(Box::new(a))
}

fn eval(expr: &Expr) -> i64 {
    todo!()
}

// Should build: (2 + 3) * -(4)
fn example() -> Expr {
    todo!()
}

fn main() {
    println!("{:?} = {}", example(), eval(&example()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_number() {
        assert_eq!(eval(&Num(7)), 7);
    }

    #[test]
    fn nested() {
        // 1 + (2 * 3)
        let e = add(Num(1), mul(Num(2), Num(3)));
        assert_eq!(eval(&e), 7);
    }

    #[test]
    fn the_example() {
        assert_eq!(eval(&example()), -20);
    }
}

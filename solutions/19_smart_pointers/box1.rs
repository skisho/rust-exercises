// box1.rs - solution
//
// A `Box<Expr>` is just a pointer, so `Expr` now has a known, fixed size.
// When matching on `&Expr`, the bindings are `&Box<Expr>`; passing them to
// `eval` (which wants `&Expr`) works thanks to automatic deref. The
// recursion in `eval` mirrors the recursion in the data.

#[derive(Debug)]
enum Expr {
    Num(i64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
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
    match expr {
        Num(n) => *n,
        Add(a, b) => eval(a) + eval(b),
        Mul(a, b) => eval(a) * eval(b),
        Neg(a) => -eval(a),
    }
}

fn example() -> Expr {
    mul(add(Num(2), Num(3)), neg(Num(4)))
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

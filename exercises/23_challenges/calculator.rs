// calculator.rs (challenge)
//
// Write a calculator that understands normal "infix" arithmetic, with
// operator precedence and parentheses:
//
//     "2 * (3 + 4) - -1"  =>  15
//
// This is a small version of what every compiler does, in two stages:
//
//   1. LEXING: `tokenize` turns the text into a list of tokens, skipping
//      whitespace. Numbers may have a decimal point ("2.5").
//
//   2. PARSING + EVALUATING: `evaluate` walks the tokens using this
//      grammar. Each rule becomes one method on a parser struct that
//      tracks its position in the token list ("recursive descent"):
//
//        expr   = term   (("+" | "-") term)*
//        term   = factor (("*" | "/") factor)*
//        factor = NUMBER | "-" factor | "(" expr ")"
//
//      Precedence falls out of the structure: `term` binds tighter than
//      `expr`, so `*` happens before `+`. The `*` loops make operators
//      left-associative: 10 - 4 - 3 = (10 - 4) - 3.
//
// Errors (see the tests for the exact cases):
//   - UnexpectedChar(c): a character that can't start any token
//   - InvalidNumber(s):  digits and dots that don't parse, like "1.2.3"
//   - UnexpectedEnd:     input ran out while something was still expected
//   - UnexpectedToken(t): a token that doesn't fit the grammar here,
//                         including leftovers after a complete expression
//   - DivisionByZero
//
// Further reading:
//   - Crafting Interpreters, "Parsing Expressions" (the clearest
//     introduction to recursive descent):
//     https://craftinginterpreters.com/parsing-expressions.html
//   - Recursive descent parsers: https://en.wikipedia.org/wiki/Recursive_descent_parser
//   - Peekable (handy for the lexer): https://doc.rust-lang.org/std/iter/struct.Peekable.html
//   - Next level: Pratt parsing, an elegant alternative, explained in Rust:
//     https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html

// I AM NOT DONE

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Num(f64),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
}

#[derive(Debug, PartialEq)]
enum CalcError {
    UnexpectedChar(char),
    InvalidNumber(String),
    UnexpectedEnd,
    UnexpectedToken(Token),
    DivisionByZero,
}

fn tokenize(input: &str) -> Result<Vec<Token>, CalcError> {
    todo!()
}

// TODO: a parser struct holding the tokens and a position, with one
// method per grammar rule.

fn evaluate(input: &str) -> Result<f64, CalcError> {
    todo!()
}

fn main() {
    for expr in ["2 * (3 + 4) - -1", "1 / 0", "(1 + 2"] {
        println!("{expr} => {:?}", evaluate(expr));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Token::*;

    #[test]
    fn tokenizes() {
        assert_eq!(
            tokenize("1 + 2.5*(3)"),
            Ok(vec![
                Num(1.0),
                Plus,
                Num(2.5),
                Star,
                LParen,
                Num(3.0),
                RParen
            ])
        );
        assert_eq!(tokenize("  "), Ok(vec![]));
    }

    #[test]
    fn tokenize_errors() {
        assert_eq!(tokenize("1 $ 2"), Err(CalcError::UnexpectedChar('$')));
        assert_eq!(
            tokenize("1.2.3"),
            Err(CalcError::InvalidNumber("1.2.3".to_string()))
        );
    }

    #[test]
    fn numbers() {
        assert_eq!(evaluate("42"), Ok(42.0));
        assert_eq!(evaluate("0.5"), Ok(0.5));
    }

    #[test]
    fn precedence() {
        assert_eq!(evaluate("1 + 2 * 3"), Ok(7.0));
        assert_eq!(evaluate("2 * 3 + 1"), Ok(7.0));
        assert_eq!(evaluate("1 + 6 / 2 - 1"), Ok(3.0));
    }

    #[test]
    fn left_associative() {
        assert_eq!(evaluate("10 - 4 - 3"), Ok(3.0));
        assert_eq!(evaluate("8 / 2 / 2"), Ok(2.0));
    }

    #[test]
    fn parentheses() {
        assert_eq!(evaluate("(1 + 2) * 3"), Ok(9.0));
        assert_eq!(evaluate("((2))"), Ok(2.0));
        assert_eq!(evaluate("2 * (3 + (4 - 1)) / 3"), Ok(4.0));
    }

    #[test]
    fn unary_minus() {
        assert_eq!(evaluate("-4 + 10 / 4"), Ok(-1.5));
        assert_eq!(evaluate("2 * -3"), Ok(-6.0));
        assert_eq!(evaluate("--5"), Ok(5.0));
        assert_eq!(evaluate("-(1 + 2)"), Ok(-3.0));
    }

    #[test]
    fn errors() {
        assert_eq!(evaluate(""), Err(CalcError::UnexpectedEnd));
        assert_eq!(evaluate("1 +"), Err(CalcError::UnexpectedEnd));
        assert_eq!(evaluate("(1 + 2"), Err(CalcError::UnexpectedEnd));
        assert_eq!(evaluate("1 2"), Err(CalcError::UnexpectedToken(Num(2.0))));
        assert_eq!(evaluate("()"), Err(CalcError::UnexpectedToken(RParen)));
        assert_eq!(evaluate("1 + 2)"), Err(CalcError::UnexpectedToken(RParen)));
        assert_eq!(evaluate("1 / (2 - 2)"), Err(CalcError::DivisionByZero));
        assert_eq!(evaluate("1 # 2"), Err(CalcError::UnexpectedChar('#')));
    }
}

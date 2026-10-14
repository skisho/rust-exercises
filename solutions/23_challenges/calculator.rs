// calculator.rs (challenge) - solution
//
// The lexer uses a Peekable iterator so it can look at the next char
// without consuming it, which is how it knows where a number ends.
//
// The parser has one method per grammar rule. Each rule calls the rule
// for the next-higher precedence level, then loops while it sees one of
// its own operators. `factor` recurses back into `expr` for parentheses,
// which is what makes nesting work to any depth.

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
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        let token = match c {
            c if c.is_whitespace() => {
                chars.next();
                continue;
            }
            '0'..='9' | '.' => {
                let mut text = String::new();
                while let Some(&d) = chars.peek() {
                    if d.is_ascii_digit() || d == '.' {
                        text.push(d);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let n = text.parse().map_err(|_| CalcError::InvalidNumber(text))?;
                tokens.push(Token::Num(n));
                continue;
            }
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '/' => Token::Slash,
            '(' => Token::LParen,
            ')' => Token::RParen,
            other => return Err(CalcError::UnexpectedChar(other)),
        };
        tokens.push(token);
        chars.next();
    }

    Ok(tokens)
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.pos).cloned();
        self.pos += 1;
        token
    }

    // expr = term (("+" | "-") term)*
    fn expr(&mut self) -> Result<f64, CalcError> {
        let mut value = self.term()?;
        loop {
            match self.peek() {
                Some(Token::Plus) => {
                    self.advance();
                    value += self.term()?;
                }
                Some(Token::Minus) => {
                    self.advance();
                    value -= self.term()?;
                }
                _ => return Ok(value),
            }
        }
    }

    // term = factor (("*" | "/") factor)*
    fn term(&mut self) -> Result<f64, CalcError> {
        let mut value = self.factor()?;
        loop {
            match self.peek() {
                Some(Token::Star) => {
                    self.advance();
                    value *= self.factor()?;
                }
                Some(Token::Slash) => {
                    self.advance();
                    let divisor = self.factor()?;
                    if divisor == 0.0 {
                        return Err(CalcError::DivisionByZero);
                    }
                    value /= divisor;
                }
                _ => return Ok(value),
            }
        }
    }

    // factor = NUMBER | "-" factor | "(" expr ")"
    fn factor(&mut self) -> Result<f64, CalcError> {
        match self.advance() {
            Some(Token::Num(n)) => Ok(n),
            Some(Token::Minus) => Ok(-self.factor()?),
            Some(Token::LParen) => {
                let value = self.expr()?;
                match self.advance() {
                    Some(Token::RParen) => Ok(value),
                    Some(other) => Err(CalcError::UnexpectedToken(other)),
                    None => Err(CalcError::UnexpectedEnd),
                }
            }
            Some(other) => Err(CalcError::UnexpectedToken(other)),
            None => Err(CalcError::UnexpectedEnd),
        }
    }
}

fn evaluate(input: &str) -> Result<f64, CalcError> {
    let mut parser = Parser { tokens: tokenize(input)?, pos: 0 };
    let value = parser.expr()?;
    match parser.advance() {
        None => Ok(value),
        Some(leftover) => Err(CalcError::UnexpectedToken(leftover)),
    }
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
            Ok(vec![Num(1.0), Plus, Num(2.5), Star, LParen, Num(3.0), RParen])
        );
        assert_eq!(tokenize("  "), Ok(vec![]));
    }

    #[test]
    fn tokenize_errors() {
        assert_eq!(tokenize("1 $ 2"), Err(CalcError::UnexpectedChar('$')));
        assert_eq!(tokenize("1.2.3"), Err(CalcError::InvalidNumber("1.2.3".to_string())));
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

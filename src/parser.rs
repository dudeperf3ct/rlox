use crate::{
    ast::{
        Expr::{self},
        UnaryOp,
    },
    errors::{ParserError, ParserResult},
    lexer::Token,
    token_type::TokenKind::{self},
};

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    fn new(mut tokens: Vec<Token>) -> Parser {
        // Since we are using vec of tokens, reverse the tokens when constructing the parser
        tokens.reverse();
        Parser { tokens }
    }

    fn next_token(&mut self) -> ParserResult<Token> {
        self.tokens.pop().ok_or_else(|| ParserError::MissingToken)
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.last()
    }

    fn is_eof(&self) -> bool {
        self.tokens.is_empty()
    }

    // primary → INTEGER | FLOAT | STRING | "true" | "false" | "nil"
    fn primary(&mut self) -> ParserResult<Expr> {
        let token = self.next_token()?;
        let expr = match token.kind {
            TokenKind::StringLiteral(x) => Expr::String(x),
            TokenKind::INumber(x) => Expr::Int(x),
            TokenKind::FNumber(x) => Expr::Float(x),
            _ => return Err(ParserError::ExpectExpressionFound(token.lexeme)),
        };
        Ok(expr)
    }

    // factor → unary (("*" | "/") unary)*
    fn factor(&mut self) -> ParserResult<Expr> {
        let mut ast = self.unary()?;
        while let Some(token) = self.peek() {
            match token.kind {
                TokenKind::Slash | TokenKind::Star => {
                    let binop = token.kind.clone().into();
                    self.next_token()?;
                    let right = self.unary()?;
                    ast = Expr::Binary(Box::new(ast), binop, Box::new(right))
                }
                _ => break,
            }
        }
        Ok(ast)
    }

    // unary → ("!" | "-") unary | primary
    fn unary(&mut self) -> ParserResult<Expr> {
        match self.peek() {
            Some(token) if (token.kind == TokenKind::Minus) || (token.kind == TokenKind::Bang) => {
                let uop = match token.kind {
                    TokenKind::Bang => UnaryOp::Not,
                    TokenKind::Minus => UnaryOp::Minus,
                    _ => unreachable!(),
                };
                self.next_token()?;
                let ast = self.unary()?;
                Ok(Expr::Unary(uop, Box::new(ast)))
            }
            _ => self.primary(),
        }
    }

    // term → factor (("+" | "-") factor)*
    fn term(&mut self) -> ParserResult<Expr> {
        let mut ast = self.factor()?;
        while let Some(token) = self.peek() {
            match token.kind {
                TokenKind::Plus | TokenKind::Minus => {
                    let binop = token.kind.clone().into();
                    self.next_token()?;
                    let right = self.factor()?;
                    ast = Expr::Binary(Box::new(ast), binop, Box::new(right))
                }
                _ => break,
            }
        }
        Ok(ast)
    }

    // expression -> term
    fn parse(&mut self) -> ParserResult<Expr> {
        let ast = self.term()?;
        let token = self.next_token()?;
        if token.kind != TokenKind::Eof || !self.tokens.is_empty() {
            return Err(ParserError::ExpectExpressionFound(token.lexeme));
        }
        Ok(ast)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;

    macro_rules! test_parse {
        ($name:ident, $source:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let tokens = tokenize($source).unwrap();
                let mut parser = Parser::new(tokens);

                assert_eq!(parser.parse().unwrap(), $expected);
            }
        };

        (error $name:ident, $source:expr) => {
            #[test]
            fn $name() {
                let tokens = tokenize($source).unwrap();
                let mut parser = Parser::new(tokens);

                assert!(
                    parser.parse().is_err(),
                    "Expected parsing to fail for: {:?}",
                    $source
                );
            }
        };
    }

    fn binary(left: Expr, op: TokenKind, right: Expr) -> Expr {
        Expr::Binary(Box::new(left), op.into(), Box::new(right))
    }

    fn unary(op: UnaryOp, expr: Expr) -> Expr {
        Expr::Unary(op, Box::new(expr))
    }

    test_parse!(error parse_missing_operand, "1 +");

    test_parse!(error parse_missing_left_operand, "* 2");

    test_parse!(error parse_consecutive_literals, "1 2");

    test_parse!(error parse_invalid_operator_sequence, "1 + * 2");

    test_parse!(error parse_empty_expression, "");

    test_parse!(parse_integer, "42", Expr::Int(42));

    test_parse!(parse_float, "3.14", Expr::Float(3.14));

    test_parse!(
        parse_string,
        r#""hello world""#,
        Expr::String("hello world".to_string())
    );

    test_parse!(
        parse_unary_minus,
        "-42",
        unary(UnaryOp::Minus, Expr::Int(42))
    );

    test_parse!(
        parse_nested_unary,
        "--42",
        unary(UnaryOp::Minus, unary(UnaryOp::Minus, Expr::Int(42)))
    );

    test_parse!(
        parse_not_and_minus,
        "!-42",
        unary(UnaryOp::Not, unary(UnaryOp::Minus, Expr::Int(42)))
    );

    test_parse!(
        parse_addition,
        "1 + 2",
        binary(Expr::Int(1), TokenKind::Plus, Expr::Int(2))
    );

    test_parse!(
        parse_multiplication,
        "2 * 3",
        binary(Expr::Int(2), TokenKind::Star, Expr::Int(3))
    );

    test_parse!(
        parse_left_associativity,
        "10 - 5 - 2",
        binary(
            binary(Expr::Int(10), TokenKind::Minus, Expr::Int(5)),
            TokenKind::Minus,
            Expr::Int(2)
        )
    );

    test_parse!(
        parse_operator_precedence,
        "1 + 2 * 3",
        binary(
            Expr::Int(1),
            TokenKind::Plus,
            binary(Expr::Int(2), TokenKind::Star, Expr::Int(3))
        )
    );

    test_parse!(
        parse_operator_precedence_reverse,
        "1 * 2 + 3",
        binary(
            binary(Expr::Int(1), TokenKind::Star, Expr::Int(2)),
            TokenKind::Plus,
            Expr::Int(3)
        )
    );
}

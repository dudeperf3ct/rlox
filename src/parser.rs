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

    fn expect(&mut self, expected: TokenKind, err: &str) -> Result<(), ParserError> {
        match self.peek() {
            Some(actual) if actual.kind == expected => {
                self.next_token()?;
                Ok(())
            }
            Some(actual) => Err(ParserError::UnexpectedToken(actual.clone(), err.into())),
            _ => Err(ParserError::MissingTokenMsg(err.into())),
        }
    }

    // primary → INTEGER | FLOAT | STRING | true | false | nil | grouping
    fn primary(&mut self) -> ParserResult<Expr> {
        let token = self.next_token()?;
        let expr = match token.kind {
            TokenKind::StringLiteral(x) => Expr::String(x),
            TokenKind::INumber(x) => Expr::Int(x),
            TokenKind::FNumber(x) => Expr::Float(x),
            TokenKind::KwNil => Expr::Nil,
            TokenKind::KwTrue => Expr::Boolean(true),
            TokenKind::KwFalse => Expr::Boolean(false),
            TokenKind::OpenParen => {
                let expr = self.expression()?;
                self.expect(TokenKind::CloseParen, "expected ) after expression")?;
                Expr::Grouping(Box::new(expr))
            }
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

    // comparison -> term ((">" | ">=" | "<" | "<=") term)*
    fn comparison(&mut self) -> ParserResult<Expr> {
        let mut ast = self.term()?;
        while let Some(token) = self.peek() {
            match token.kind {
                TokenKind::Gt | TokenKind::GtEq | TokenKind::Lt | TokenKind::LtEq => {
                    let binop = token.kind.clone().into();
                    self.next_token()?;
                    let right = self.term()?;
                    ast = Expr::Binary(Box::new(ast), binop, Box::new(right))
                }
                _ => break,
            }
        }
        Ok(ast)
    }

    // expression -> comparison
    fn expression(&mut self) -> ParserResult<Expr> {
        self.comparison()
    }

    // Parser convert tokens into expression
    fn parse(&mut self) -> ParserResult<Expr> {
        let ast = self.expression()?;
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

    test_parse!(
        parse_grouping,
        "(1 + 2) * 3",
        binary(
            Expr::Grouping(Box::new(binary(
                Expr::Int(1),
                TokenKind::Plus,
                Expr::Int(2)
            ))),
            TokenKind::Star,
            Expr::Int(3)
        )
    );

    test_parse!(
        parse_grouping_overrides_precedence,
        "1 * (2 + 3)",
        binary(
            Expr::Int(1),
            TokenKind::Star,
            Expr::Grouping(Box::new(binary(
                Expr::Int(2),
                TokenKind::Plus,
                Expr::Int(3)
            )))
        )
    );

    test_parse!(
        parse_nested_grouping,
        "((1 + 2))",
        Expr::Grouping(Box::new(Expr::Grouping(Box::new(binary(
            Expr::Int(1),
            TokenKind::Plus,
            Expr::Int(2)
        )))))
    );

    test_parse!(error parse_invalid_group, "(1 + 2");

    test_parse!(error parse_empty_group, "()");

    test_parse!(error parse_extra_closing_paren, "(1 + 2))");

    test_parse!(error parse_missing_inner_paren, "((1 + 2)");

    test_parse!(parse_true_literal, "true", Expr::Boolean(true));

    test_parse!(parse_false_literal, "false", Expr::Boolean(false));

    test_parse!(parse_nil_literal, "nil", Expr::Nil);

    test_parse!(parse_integer, "42", Expr::Int(42));

    test_parse!(parse_float, "3.14", Expr::Float(3.14));

    test_parse!(
        parse_unary_boolean,
        "!false",
        unary(UnaryOp::Not, Expr::Boolean(false))
    );

    test_parse!(
        parse_comparison_greater_than,
        "3 > 2",
        binary(Expr::Int(3), TokenKind::Gt, Expr::Int(2))
    );

    test_parse!(
        parse_comparison_precedence,
        "1 + 2 >= 3 * 4",
        binary(
            binary(Expr::Int(1), TokenKind::Plus, Expr::Int(2)),
            TokenKind::GtEq,
            binary(Expr::Int(3), TokenKind::Star, Expr::Int(4))
        )
    );

    test_parse!(
        parse_comparison_chained,
        "1 < 2 <= 3",
        binary(
            binary(Expr::Int(1), TokenKind::Lt, Expr::Int(2)),
            TokenKind::LtEq,
            Expr::Int(3)
        )
    );

    test_parse!(
        error parse_comparison_missing_operand,
        "1 >"
    );
}

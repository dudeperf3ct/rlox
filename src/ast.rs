use crate::{
    errors::{ParserError, ParserResult},
    lexer::Token,
    token_type::TokenKind,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Literal(i64),

    Binary {
        left: Box<Expr>,
        right: Box<Expr>,
        operator: TokenKind,
    },
}

fn parse(inputs: &[Token]) -> ParserResult<Expr> {
    let mut expression: Option<Expr> = None;
    let mut tokens = inputs.iter();
    while let Some(token) = tokens.next() {
        match token.kind {
            TokenKind::INumber(x) => expression = Some(Expr::Literal(x)),
            TokenKind::Eof => return expression.ok_or(ParserError),
            _ => return Err(ParserError),
        };
    }
    Err(ParserError)
}

#[cfg(test)]
mod tests {
    use crate::lexer::tokenize;

    use super::*;

    #[test]
    fn parse_integer_literal() {
        let tokens = tokenize("42").unwrap();
        let ast = parse(tokens.as_slice());
        let expected = Expr::Literal(42);

        assert_eq!(ast.unwrap(), expected);
    }
}

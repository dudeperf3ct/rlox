use crate::token_type::TokenKind;
use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum UnaryOp {
    Minus,
    Not,
}

impl Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            UnaryOp::Minus => "-",
            UnaryOp::Not => "!",
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Plus,
    Minus,
    Star,
    Slash,
    Gt,
    Lt,
    Eq,
    LtEq,
    GtEq,
    EqEq,
    BangEq,
    And,
    Or,
}

impl From<TokenKind> for BinaryOp {
    fn from(kind: TokenKind) -> Self {
        match kind {
            TokenKind::Plus => BinaryOp::Plus,
            TokenKind::Minus => BinaryOp::Minus,
            TokenKind::Star => BinaryOp::Star,
            TokenKind::Slash => BinaryOp::Slash,
            TokenKind::Lt => BinaryOp::Lt,
            TokenKind::Gt => BinaryOp::Gt,
            TokenKind::Eq => BinaryOp::Eq,
            TokenKind::LtEq => BinaryOp::LtEq,
            TokenKind::GtEq => BinaryOp::GtEq,
            TokenKind::EqEq => BinaryOp::EqEq,
            TokenKind::BangEq => BinaryOp::BangEq,
            TokenKind::KwAnd => BinaryOp::And,
            TokenKind::KwOr => BinaryOp::Or,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Int(i64),
    Float(f64),
    Boolean(bool),
    String(String),

    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    Unary(UnaryOp, Box<Expr>),
    Grouping(Box<Expr>),
}

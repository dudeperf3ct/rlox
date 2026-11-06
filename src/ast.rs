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
    LtEq,
    GtEq,
    EqEq,
    BangEq,
    And,
    Or,
}

impl Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            BinaryOp::Plus => "+",
            BinaryOp::Minus => "-",
            BinaryOp::Star => "*",
            BinaryOp::Slash => "/",
            BinaryOp::Lt => "<",
            BinaryOp::Gt => ">",
            BinaryOp::EqEq => "==",
            BinaryOp::LtEq => "<=",
            BinaryOp::GtEq => ">=",
            BinaryOp::BangEq => "!=",
            BinaryOp::Or => "or",
            BinaryOp::And => "and",
        };
        write!(f, "{}", c)
    }
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
            TokenKind::LtEq => BinaryOp::LtEq,
            TokenKind::GtEq => BinaryOp::GtEq,
            TokenKind::EqEq => BinaryOp::EqEq,
            TokenKind::BangEq => BinaryOp::BangEq,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogicalOp {
    And,
    Or,
}

impl From<TokenKind> for LogicalOp {
    fn from(kind: TokenKind) -> Self {
        match kind {
            TokenKind::KwAnd => LogicalOp::And,
            TokenKind::KwOr => LogicalOp::Or,
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
    Nil,

    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    Unary(UnaryOp, Box<Expr>),
    Grouping(Box<Expr>),
    Logical(Box<Expr>, LogicalOp, Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Single characater symbols
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Comma,
    Dot,
    Semicolon,
    Slash,

    // BinOps
    Minus,
    Plus,
    Star,
    And,
    Or,

    // Multi character
    Eq,
    EqEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    Bang,
    BangEq,

    Ident(String),
    StringLiteral(String),
    INumber(i64),
    FNumber(f64),

    // Keywords
    KwFunc,
    KwResult,
    KwSuper,
    KwClass,
    KwPrint,
    KwVar,
    KwFor,
    KwWhile,
    KwIf,
    KwElse,
    KwTrue,
    KwFalse,
    KwNil,

    Eof,
}

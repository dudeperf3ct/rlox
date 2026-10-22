use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Single character symbols
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

    // Literal, Identifiers and Numerical
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
    KwAnd,
    KwOr,

    Eof,
}

pub fn get_keyword_mapping() -> HashMap<String, TokenKind> {
    let mut reserved_keywords = HashMap::new();
    reserved_keywords.insert("func".to_string(), TokenKind::KwFunc);
    reserved_keywords.insert("return".to_string(), TokenKind::KwResult);
    reserved_keywords.insert("super".to_string(), TokenKind::KwSuper);
    reserved_keywords.insert("class".to_string(), TokenKind::KwClass);
    reserved_keywords.insert("print".to_string(), TokenKind::KwPrint);
    reserved_keywords.insert("var".to_string(), TokenKind::KwVar);
    reserved_keywords.insert("for".to_string(), TokenKind::KwFor);
    reserved_keywords.insert("while".to_string(), TokenKind::KwWhile);
    reserved_keywords.insert("if".to_string(), TokenKind::KwIf);
    reserved_keywords.insert("else".to_string(), TokenKind::KwElse);
    reserved_keywords.insert("true".to_string(), TokenKind::KwTrue);
    reserved_keywords.insert("false".to_string(), TokenKind::KwFalse);
    reserved_keywords.insert("nil".to_string(), TokenKind::KwNil);
    reserved_keywords.insert("and".to_string(), TokenKind::KwAnd);
    reserved_keywords.insert("or".to_string(), TokenKind::KwOr);
    reserved_keywords
}

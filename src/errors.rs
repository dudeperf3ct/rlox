use std::fmt;

use crate::lexer::Location;
use thiserror::Error;

pub type LexerResult<T> = Result<T, LexerError>;
pub type ParserResult<T> = Result<T, ParserError>;

// TODO: Convert LexerError to thiserror
#[derive(Debug)]
pub struct LexerError {
    message: String,
    location: Option<Location>,
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(loc) = self.location {
            write!(f, "{}:{} ", loc.line + 1, loc.column + 1)?
        }
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for LexerError {}

impl LexerError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            location: None,
        }
    }

    pub fn with_location(self, location: Location) -> LexerError {
        LexerError {
            location: Some(location),
            ..self
        }
    }
}

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("Error: missing token.")]
    MissingToken,

    #[error("Error at '{0}': Expect expression.")]
    ExpectExpressionFound(String),
}

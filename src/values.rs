use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    Nil,
    String(String),
    Boolean(bool),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Int(i) => write!(f, "{}", *i),
            Value::Float(fl) => write!(f, "{}", *fl),
            Value::Boolean(b) => write!(f, "{}", *b),
            Value::String(s) => write!(f, "\"{}\"", s),
        }
    }
}

impl Value {
    pub fn is_truth(&self) -> bool {
        // Treat Nil as false
        !matches!(self, Value::Nil | Value::Boolean(false))
    }
}

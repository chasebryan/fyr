use std::fmt::{Display, Formatter};

use crate::span::Span;

pub type FyrResult<T> = Result<T, FyrError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FyrError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl FyrError {
    pub fn new(message: impl Into<String>, span: Span) -> Self {
        Self {
            message: message.into(),
            line: span.line,
            column: span.column,
        }
    }
}

impl Display for FyrError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "fyr error at {}:{}: {}",
            self.line, self.column, self.message
        )
    }
}

impl std::error::Error for FyrError {}

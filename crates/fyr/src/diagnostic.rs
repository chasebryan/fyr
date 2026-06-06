use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};

use crate::span::Span;

pub type FyrResult<T> = Result<T, FyrError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FyrError {
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub file: Option<PathBuf>,
}

impl FyrError {
    pub fn new(message: impl Into<String>, span: Span) -> Self {
        Self {
            message: message.into(),
            line: span.line,
            column: span.column,
            file: None,
        }
    }

    pub fn with_fallback_span(mut self, span: Span) -> Self {
        if self.line == 0 && self.column == 0 {
            self.line = span.line;
            self.column = span.column;
        }

        self
    }

    pub fn with_fallback_source_path(mut self, source_path: Option<&Path>) -> Self {
        if self.file.is_none()
            && let Some(source_path) = source_path
        {
            self.file = Some(source_path.to_path_buf());
        }

        self
    }

    pub fn with_fallback_location(self, span: Span, source_path: Option<&Path>) -> Self {
        self.with_fallback_span(span)
            .with_fallback_source_path(source_path)
    }
}

impl Display for FyrError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(file) = &self.file {
            write!(
                f,
                "fyr error at {}:{}:{}: {}",
                file.display(),
                self.line,
                self.column,
                self.message
            )
        } else {
            write!(
                f,
                "fyr error at {}:{}: {}",
                self.line, self.column, self.message
            )
        }
    }
}

impl std::error::Error for FyrError {}

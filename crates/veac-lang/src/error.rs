use crate::token::Span;
use std::fmt;

/// All error types for VEAC compilation.
#[derive(Debug, Clone)]
pub struct VeacError {
    pub kind: ErrorKind,
    pub message: String,
    pub span: Option<Span>,
    pub hint: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    // Lexer errors
    UnexpectedChar,
    UnterminatedString,
    InvalidNumber,
    InvalidTimeLiteral,
    InvalidColor,

    // Parser errors
    UnexpectedToken,
    ExpectedToken,
    ExpectedBlock,
    ExpectedExpression,

    // Semantic errors
    UndefinedAsset,
    UndefinedVariable,
    DuplicateDefinition,
    TypeMismatch,
    InvalidTimeRange,
    InvalidValue,
    AssetFileNotFound,
    NoTimeline,
    NoProject,
}

impl VeacError {
    pub fn new(kind: ErrorKind, message: impl Into<String>, span: Option<Span>) -> Self {
        Self {
            kind,
            message: message.into(),
            span,
            hint: None,
        }
    }

    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    /// Format the error with source context (rustc-style).
    pub fn format(&self, source: &str, filename: &str) -> String {
        let mut out = String::new();

        // Error header
        out.push_str(&format!("error[{:?}]: {}\n", self.kind, self.message));

        if let Some(span) = &self.span {
            // Location line
            out.push_str(&format!("  --> {}:{}:{}\n", filename, span.line, span.col));

            // Source context
            let lines: Vec<&str> = source.lines().collect();
            if span.line > 0 && span.line <= lines.len() {
                let line_str = lines[span.line - 1];
                let line_num = format!("{}", span.line);
                let padding = " ".repeat(line_num.len());

                out.push_str(&format!("{padding} |\n"));
                out.push_str(&format!("{line_num} | {line_str}\n"));

                // Underline
                let underline_start = if span.col > 0 { span.col - 1 } else { 0 };
                let underline_len = (span.end - span.start).max(1);
                out.push_str(&format!(
                    "{padding} | {}{}\n",
                    " ".repeat(underline_start),
                    "^".repeat(underline_len)
                ));
            }
        }

        if let Some(hint) = &self.hint {
            out.push_str(&format!("  = help: {hint}\n"));
        }

        out
    }
}

impl fmt::Display for VeacError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for VeacError {}

pub type Result<T> = std::result::Result<T, VeacError>;

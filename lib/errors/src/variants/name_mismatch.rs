use std::fmt;
use syntax::span::Span;

/// Error for mismatching names
#[derive(Debug)]
pub struct NameMismatch {
    /// Found name
    found: String,
    /// Expected name
    /// or description what was expected
    expected: String,
    /// Source location
    span: Span,
}

impl NameMismatch {
    /// Create a new error from found,expected and span
    #[must_use]
    pub fn new(found: &str, expected: &str, span: Span) -> Self {
        Self {
            found: found.to_owned(),
            expected: expected.to_owned(),
            span,
        }
    }
}

impl fmt::Display for NameMismatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Unexpected name: {}, expected: {} (at {})",
            self.found, self.expected, self.span
        )
    }
}

impl std::error::Error for NameMismatch {}

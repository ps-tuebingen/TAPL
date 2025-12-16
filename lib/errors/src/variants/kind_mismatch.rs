use std::fmt;
use syntax::span::Span;

/// Error for not matching kinds
#[derive(Debug)]
pub struct KindMismatch {
    /// found kind (as string)
    found: String,
    /// what was expected
    /// either another kind or "star kind"/"arrow kind"
    expected: String,
    /// Source location
    span: Span,
}

impl KindMismatch {
    /// Create a new error from found, expected and span
    #[must_use]
    pub const fn new(found: String, expected: String, span: Span) -> Self {
        Self {
            found,
            expected,
            span,
        }
    }
}

impl fmt::Display for KindMismatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Kind Mismatch:\n\texpected: {}\n\tfound {} (at {})",
            self.expected, self.found, self.span
        )
    }
}

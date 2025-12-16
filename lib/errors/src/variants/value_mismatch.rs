use std::fmt;
use syntax::span::Span;

/// Error for mismatching values
#[derive(Debug)]
pub struct ValueMismatch {
    /// Found value (as string)
    found: String,
    /// Expected value (as string)
    /// or description what was expected
    expected: String,
    /// Source location
    span: Span,
}

impl ValueMismatch {
    #[must_use]
    pub const fn new(found: String, expected: String, span: Span) -> Self {
        Self {
            found,
            expected,
            span,
        }
    }
}

impl fmt::Display for ValueMismatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Value Mismatch:\n\tfound {},\n\texpected {} (at {})",
            self.found, self.expected, self.span
        )
    }
}

impl std::error::Error for ValueMismatch {}

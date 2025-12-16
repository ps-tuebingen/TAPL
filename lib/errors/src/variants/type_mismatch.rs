use std::fmt;
use syntax::span::Span;

/// Error for mismatching types
#[derive(Debug)]
pub struct TypeMismatch {
    /// Found Type (as string)
    found: String,
    /// Expected Type (as string)
    /// or description what was expected
    expected: String,
    /// Source location
    span: Span,
}

impl TypeMismatch {
    #[must_use]
    pub const fn new(found: String, expected: String, span: Span) -> Self {
        Self {
            found,
            expected,
            span,
        }
    }
}

impl fmt::Display for TypeMismatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Type Mismatch:\n\texpected: {}, found: {} (at {})",
            self.expected, self.found, self.span
        )
    }
}

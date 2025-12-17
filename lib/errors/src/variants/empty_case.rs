use std::fmt;
use syntax::span::Span;

/// Error when a match expression has no patterns
#[derive(Debug)]
pub struct EmptyCase {
    /// Source location
    span: Span,
}

impl EmptyCase {
    /// Create a new error from span
    #[must_use]
    pub const fn new(span: Span) -> Self {
        Self { span }
    }
}

impl fmt::Display for EmptyCase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cannot have zero patterns in match (at {})", self.span)
    }
}

impl std::error::Error for EmptyCase {}

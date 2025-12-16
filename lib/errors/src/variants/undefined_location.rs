use std::fmt;
use syntax::span::Span;

/// Access of undefined memory location
#[derive(Debug)]
pub struct UndefinedLocation {
    /// Accessed location
    loc: usize,
    /// Source location
    span: Span,
}

impl UndefinedLocation {
    /// Create a new error from location and span
    #[must_use]
    pub const fn new(loc: usize, span: Span) -> Self {
        Self { loc, span }
    }
}

impl fmt::Display for UndefinedLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Undefined Location {} (at {})", self.loc, self.span)
    }
}

impl std::error::Error for UndefinedLocation {}

use std::fmt;
use syntax::span::Span;

/// Array index outside of bounds
/// usually for [`syntax::terms::tuple::Tuple`]
#[derive(Debug)]
pub struct IndexOutOfBounds {
    /// Accessed index
    tried: usize,
    /// Actual length
    len: usize,
    /// Source location
    span: Span,
}

impl IndexOutOfBounds {
    /// Create a new error from tried index, length and span
    #[must_use]
    pub const fn new(tried: usize, len: usize, span: Span) -> Self {
        Self { tried, len, span }
    }
}

impl fmt::Display for IndexOutOfBounds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Index {} is out of bounds (length is {}, at {})",
            self.tried, self.len, self.span
        )
    }
}

impl std::error::Error for IndexOutOfBounds {}

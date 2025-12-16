use std::fmt;
use syntax::{TypeVar, span::Span};

/// Error when encountering a unbound type variable
#[derive(Debug)]
pub struct FreeTypeVariable {
    /// The variable
    var: TypeVar,
    /// Source Location
    span: Span,
}

impl FreeTypeVariable {
    /// Create a new error from variable and span
    #[must_use]
    pub fn new(var: &str, span: Span) -> Self {
        Self {
            var: var.to_owned(),
            span,
        }
    }
}

impl fmt::Display for FreeTypeVariable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Free Type Variable {} (at {})", self.var, self.span)
    }
}

impl std::error::Error for FreeTypeVariable {}

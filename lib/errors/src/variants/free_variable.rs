use std::fmt;
use syntax::span::Span;

/// Variable was not bound
#[derive(Debug)]
pub struct FreeVariable {
    /// The variable
    var: String,
    /// Source location
    span: Span,
}

impl FreeVariable {
    /// Create a new error from variable and span
    #[must_use]
    pub fn new(var: &str, span: Span) -> Self {
        Self {
            var: var.to_owned(),
            span,
        }
    }
}

impl fmt::Display for FreeVariable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Free Variable {} (at {})", self.var, self.span)
    }
}

impl std::error::Error for FreeVariable {}

use std::fmt;
use syntax::span::Span;

/// Error when an unexpected keyword was encountered
#[derive(Debug)]
pub struct UnknownKeyword {
    /// The found keyword
    kw: String,
    /// Source location
    span: Span,
}

impl UnknownKeyword {
    /// Create a new error from keyword and span
    #[must_use]
    pub fn new(kw: &str, span: Span) -> Self {
        Self {
            kw: kw.to_owned(),
            span,
        }
    }
}

impl fmt::Display for UnknownKeyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unknown keyword {} (at {})", self.kw, self.span)
    }
}

impl std::error::Error for UnknownKeyword {}

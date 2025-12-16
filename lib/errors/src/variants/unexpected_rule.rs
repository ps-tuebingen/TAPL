use std::fmt;
use syntax::span::Span;

/// Error when an unepxected [`parser::Rule`] was encountered
#[derive(Debug)]
pub struct UnexpectedRule {
    /// Rule that was found (as string)
    found: String,
    /// Rule that was expected (as string)
    expected: String,
    /// Source Location
    span: Span,
}
impl UnexpectedRule {
    /// Create a new error from found, expected and span
    #[must_use]
    pub fn new(found: &str, expected: &str, span: Span) -> Self {
        Self {
            found: found.to_owned(),
            expected: expected.to_owned(),
            span,
        }
    }
}

impl fmt::Display for UnexpectedRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Unexpected rule {:?}, expected: {} (at {})",
            self.found, self.expected, self.span
        )
    }
}

impl std::error::Error for UnexpectedRule {}

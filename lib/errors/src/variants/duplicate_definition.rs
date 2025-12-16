use std::fmt;
use syntax::span::Span;

/// Program contains the same definition multiple times
#[derive(Debug)]
pub struct DuplicateDefinition {
    /// Name of the definition
    name: String,
    /// Source location
    span: Span,
}

impl DuplicateDefinition {
    /// Create a new error from name and span
    #[must_use]
    pub fn new(name: &str, span: Span) -> Self {
        Self {
            name: name.to_owned(),
            span,
        }
    }
}

impl fmt::Display for DuplicateDefinition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} was defined multiple times (at {})",
            self.name, self.span
        )
    }
}

impl std::error::Error for DuplicateDefinition {}

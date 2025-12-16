use std::fmt;
use syntax::span::Span;

/// Label was not defined
/// usually for [`syntax::terms::record::Record`] or [`syntax::terms::variant::Variant`]
#[derive(Debug)]
pub struct UndefinedLabel {
    /// Used label
    label: String,
    /// Source location
    span: Span,
}

impl UndefinedLabel {
    /// Create a new error from label and span
    #[must_use]
    pub fn new(lb: &str, span: Span) -> Self {
        Self {
            label: lb.to_owned(),
            span,
        }
    }
}

impl fmt::Display for UndefinedLabel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Undefined Label {} (at {})", self.label, self.span)
    }
}

impl std::error::Error for UndefinedLabel {}

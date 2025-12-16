use std::fmt;
use syntax::span::Span;

/// Error when subtyping could not be checked
#[derive(Debug)]
pub struct NotASubtype {
    /// Subtype
    sub: String,
    /// Supertype
    sup: String,
    /// Source location
    span: Span,
}

impl NotASubtype {
    pub fn new<Ty1, Ty2>(sub_ty: Ty1, super_ty: Ty2, span: Span) -> Self
    where
        Ty1: fmt::Display,
        Ty2: fmt::Display,
    {
        Self {
            sub: sub_ty.to_string(),
            sup: super_ty.to_string(),
            span,
        }
    }
}

impl fmt::Display for NotASubtype {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} is not a subtype of {} (at {})",
            self.sub, self.sup, self.span
        )
    }
}

impl std::error::Error for NotASubtype {}

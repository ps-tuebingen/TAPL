use super::Value;
use crate::{
    language::Language,
    span::{Span, Spanned},
    terms::Nil as NilT,
};
use std::fmt;

/// Empty list value
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Nil<Lang>
where
    Lang: Language,
{
    /// Annotated type
    pub ty: Lang::Type,
    /// Source location
    pub span: Span,
}

impl<Lang> Nil<Lang>
where
    Lang: Language,
{
    /// Create a new Nil value with given type and span
    pub fn new<Ty>(ty: Ty, span: Span) -> Self
    where
        Ty: Into<Lang::Type>,
    {
        Self {
            ty: ty.into(),
            span,
        }
    }
}

impl<Lang> Spanned for Nil<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Value for Nil<Lang>
where
    Lang: Language,
    NilT<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;
    type Term = NilT<Lang>;
}

impl<Lang> From<Nil<Lang>> for NilT<Lang>
where
    Lang: Language,
{
    fn from(nil: Nil<Lang>) -> Self {
        Self::new(nil.ty, nil.span)
    }
}

impl<Lang> fmt::Display for Nil<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Nil[{}]", self.ty)
    }
}

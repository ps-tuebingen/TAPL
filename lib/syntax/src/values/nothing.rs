use super::Value;
use crate::{
    language::Language,
    span::{Span, Spanned},
    terms::Nothing as NothingT,
};
use std::fmt;

/// Nothing value
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Nothing<Lang>
where
    Lang: Language,
{
    /// Annotated type
    pub ty: Lang::Type,
    /// Source location
    pub span: Span,
}

impl<Lang> Nothing<Lang>
where
    Lang: Language,
{
    /// Create a new nothing value with given type and span
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

impl<Lang> Spanned for Nothing<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Value for Nothing<Lang>
where
    Lang: Language,
    NothingT<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;
    type Term = NothingT<Lang>;
}

impl<Lang> From<Nothing<Lang>> for NothingT<Lang>
where
    Lang: Language,
{
    fn from(not: Nothing<Lang>) -> Self {
        Self::new(not.ty, not.span)
    }
}

impl<Lang> fmt::Display for Nothing<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Nothing[{}]", self.ty)
    }
}

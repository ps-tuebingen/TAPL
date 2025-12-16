use super::Value;
use crate::{
    language::Language,
    span::{Span, Spanned},
    terms::Fold as FoldT,
};
use macros::EqNoSpan;
use std::fmt;

/// Fold value
#[derive(Debug, EqNoSpan, Clone)]
pub struct Fold<Lang>
where
    Lang: Language,
{
    /// folded type
    pub ty: Lang::Type,
    /// inner value
    pub val: Box<Lang::Value>,
    /// source location
    pub span: Span,
}

impl<Lang> Fold<Lang>
where
    Lang: Language,
{
    /// Create a new fold value with given type, inner value and span
    pub fn new<Ty1, V1>(ty: Ty1, v: V1, span: Span) -> Self
    where
        Ty1: Into<Lang::Type>,
        V1: Into<Lang::Value>,
    {
        Self {
            ty: ty.into(),
            val: Box::new(v.into()),
            span,
        }
    }
}

impl<Lang> Spanned for Fold<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Value for Fold<Lang>
where
    Lang: Language,
    FoldT<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;
    type Term = FoldT<Lang>;
}

impl<Lang> From<Fold<Lang>> for FoldT<Lang>
where
    Lang: Language,
{
    fn from(fld: Fold<Lang>) -> Self {
        Self::new(*fld.val, fld.ty, fld.span)
    }
}

impl<Lang> fmt::Display for Fold<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fold[{}]({})", self.ty, self.val)
    }
}

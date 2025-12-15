use super::Value;
use crate::{
    language::Language,
    span::{Span, Spanned},
    terms::Right as RightT,
};
use std::fmt;

/// Right injection value
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Right<Lang>
where
    Lang: Language,
{
    /// Right value
    pub right_val: Box<Lang::Value>,
    /// Annotated type
    pub ty: Lang::Type,
    /// Source location
    pub span: Span,
}

impl<Lang> Right<Lang>
where
    Lang: Language,
{
    /// Create a new right value with given inner value, type and span
    pub fn new<V1, Ty1>(val: V1, ty: Ty1, span: Span) -> Self
    where
        V1: Into<Lang::Value>,
        Ty1: Into<Lang::Type>,
    {
        Self {
            right_val: Box::new(val.into()),
            ty: ty.into(),
            span,
        }
    }
}

impl<Lang> Spanned for Right<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Value for Right<Lang>
where
    Lang: Language,
    RightT<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;
    type Term = RightT<Lang>;
}

impl<Lang> From<Right<Lang>> for RightT<Lang>
where
    Lang: Language,
{
    fn from(right: Right<Lang>) -> Self {
        Self::new(*right.right_val, right.ty, right.span)
    }
}

impl<Lang> fmt::Display for Right<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "inr({}) as {}", self.right_val, self.ty)
    }
}

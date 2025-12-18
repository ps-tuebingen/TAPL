use super::Value;
use crate::{
    language::Language,
    span::{Span, Spanned},
    terms::Something as SomethingT,
};
use macros::EqNoSpan;
use std::fmt;

/// Something value
#[derive(Debug, EqNoSpan, Clone)]
pub struct Something<Lang>
where
    Lang: Language,
{
    /// Inner value
    pub val: Box<Lang::Value>,
    /// Source location
    pub span: Span,
}

impl<Lang> Something<Lang>
where
    Lang: Language,
{
    /// Create a new Something value with given inner value and span
    pub fn new<V1>(v: V1, span: Span) -> Self
    where
        V1: Into<Lang::Value>,
    {
        Self {
            val: Box::new(v.into()),
            span,
        }
    }
}

impl<Lang> Spanned for Something<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Value for Something<Lang>
where
    Lang: Language,
    SomethingT<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;
    type Term = SomethingT<Lang>;
}

impl<Lang> From<Something<Lang>> for SomethingT<Lang>
where
    Lang: Language,
{
    fn from(something: Something<Lang>) -> Self {
        Self::new(*something.val, something.span)
    }
}

impl<Lang> fmt::Display for Something<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "something({})", self.val)
    }
}

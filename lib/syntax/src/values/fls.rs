use super::Value;
use crate::{
    language::Language,
    span::{Span, Spanned},
    terms::False as FalseT,
};
use macros::{EqNoSpan, HashNoSpan};
use std::{fmt, marker::PhantomData};

/// False Value
#[derive(HashNoSpan, Debug, EqNoSpan, Clone)]
pub struct False<Lang>
where
    Lang: Language,
{
    /// Source location
    span: Span,
    /// Save the type parameter
    phantom: PhantomData<Lang>,
}

impl<Lang> False<Lang>
where
    Lang: Language,
{
    /// Create a false value with given span
    #[must_use]
    pub const fn new(span: Span) -> Self {
        Self {
            span,
            phantom: PhantomData,
        }
    }
}

impl<Lang> Spanned for False<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Value for False<Lang>
where
    Lang: Language,
    FalseT<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;
    type Term = FalseT<Lang>;
}

impl<Lang> From<False<Lang>> for FalseT<Lang>
where
    Lang: Language,
{
    fn from(fls: False<Lang>) -> Self {
        Self::new(fls.span)
    }
}

impl<Lang> fmt::Display for False<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("false")
    }
}

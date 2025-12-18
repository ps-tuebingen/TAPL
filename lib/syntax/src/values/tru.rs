use super::Value;
use crate::{
    language::Language,
    span::{Span, Spanned},
    terms::True as TrueT,
};
use macros::{EqNoSpan, HashNoSpan};
use std::{fmt, marker::PhantomData};

/// True value
#[derive(HashNoSpan, Debug, EqNoSpan, Clone)]
pub struct True<Lang>
where
    Lang: Language,
{
    /// Source location
    pub span: Span,
    /// Save the type parameter
    phantom: PhantomData<Lang>,
}

impl<Lang> True<Lang>
where
    Lang: Language,
{
    /// Create a new true value with given span
    #[must_use]
    pub const fn new(span: Span) -> Self {
        Self {
            span,
            phantom: PhantomData,
        }
    }
}

impl<Lang> Spanned for True<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Value for True<Lang>
where
    Lang: Language,
    TrueT<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;
    type Term = TrueT<Lang>;
}

impl<Lang> From<True<Lang>> for TrueT<Lang>
where
    Lang: Language,
{
    fn from(tru: True<Lang>) -> Self {
        Self::new(tru.span)
    }
}

impl<Lang> fmt::Display for True<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("true")
    }
}

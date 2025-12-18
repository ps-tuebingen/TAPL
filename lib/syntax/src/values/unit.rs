use super::Value;
use crate::{
    language::Language,
    span::{Span, Spanned},
    terms::Unit as UnitT,
};
use macros::EqNoSpan;
use std::{fmt, marker::PhantomData};

/// Unit value
#[derive(Debug, EqNoSpan, Clone)]
pub struct Unit<Lang>
where
    Lang: Language,
{
    /// Source location
    pub span: Span,
    /// Save the type parameter
    phantom: PhantomData<Lang>,
}

impl<Lang> Unit<Lang>
where
    Lang: Language,
{
    /// Create a new unit value from a given span
    #[must_use]
    pub const fn new(span: Span) -> Self {
        Self {
            phantom: PhantomData,
            span,
        }
    }
}

impl<Lang> Spanned for Unit<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Value for Unit<Lang>
where
    Lang: Language,
    UnitT<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;
    type Term = UnitT<Lang>;
}

impl<Lang> From<Unit<Lang>> for UnitT<Lang>
where
    Lang: Language,
{
    fn from(u: Unit<Lang>) -> Self {
        Self::new(u.span)
    }
}

impl<Lang> fmt::Display for Unit<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("unit")
    }
}

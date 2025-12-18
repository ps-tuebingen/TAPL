use super::Value;
use crate::{
    language::Language,
    span::{Span, Spanned},
    terms::Loc as LocT,
};
use macros::{EqNoSpan, HashNoSpan};
use std::{fmt, marker::PhantomData};

/// Location value
#[derive(HashNoSpan, Debug, EqNoSpan, Clone)]
pub struct Loc<Lang>
where
    Lang: Language,
{
    /// Memory Location
    pub loc: usize,
    /// Source location
    pub span: Span,
    /// Save the type parameter
    phantom: PhantomData<Lang>,
}

impl<Lang> Loc<Lang>
where
    Lang: Language,
{
    /// Create a new location value from a given location and span
    #[must_use]
    pub const fn new(loc: usize, span: Span) -> Self {
        Self {
            loc,
            span,
            phantom: PhantomData,
        }
    }
}

impl<Lang> Spanned for Loc<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Value for Loc<Lang>
where
    Lang: Language,
    LocT<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;
    type Term = LocT<Lang>;
}

impl<Lang> From<Loc<Lang>> for LocT<Lang>
where
    Lang: Language,
{
    fn from(loc: Loc<Lang>) -> Self {
        Self::new(loc.loc, loc.span)
    }
}

impl<Lang> fmt::Display for Loc<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.loc)
    }
}

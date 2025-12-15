use super::Term;
use crate::{
    Location, TypeVar, Var,
    language::Language,
    span::{Span, Spanned},
    subst::{SubstTerm, SubstType},
};
use std::{fmt, marker::PhantomData};

/// Term representing a memory location
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Loc<Lang>
where
    Lang: Language,
{
    /// Memory location
    pub loc: Location,
    /// Source location
    pub span: Span,
    phantom: PhantomData<Lang>,
}

impl<Lang> Loc<Lang>
where
    Lang: Language,
{
    #[must_use]
    /// Create a new Loc with location and span
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

impl<Lang> Term for Loc<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Loc<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, _: &Var, _: &<Lang as Language>::Term) -> Self::Target {
        self
    }
}

impl<Lang> SubstType for Loc<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, _: &TypeVar, _: &<Lang as Language>::Type) -> Self::Target {
        self
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

use super::Term;
use crate::{
    TypeVar, Var,
    free_vars::{FreeTypeVars, FreeVars},
    language::Language,
    span::{Span, Spanned},
    subst::{SubstTerm, SubstType},
};
use macros::EqNoSpan;
use std::{collections::HashSet, fmt, marker::PhantomData};

/// Term representing `false`
#[derive(Clone, Debug, EqNoSpan)]
pub struct False<Lang>
where
    Lang: Language,
{
    /// saves the type parameter
    phantom: PhantomData<Lang>,
    /// Source location
    pub span: Span,
}

impl<Lang> False<Lang>
where
    Lang: Language,
{
    #[must_use]
    /// Create a new `false` with given source location
    pub const fn new(span: Span) -> Self {
        Self {
            phantom: PhantomData,
            span,
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

impl<Lang> FreeVars for False<Lang>
where
    Lang: Language,
{
    fn free_vars(&self, _: &mut HashSet<Var>) {
        ()
    }
}

impl<Lang> FreeTypeVars for False<Lang>
where
    Lang: Language,
{
    fn free_type_vars(&self, _: &mut HashSet<TypeVar>) {
        ()
    }
}

impl<Lang> Term for False<Lang> where Lang: Language {}

impl<Lang> SubstTerm for False<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, _: &Var, _: &<Lang as Language>::Term) -> Self::Target {
        self
    }
}

impl<Lang> SubstType for False<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, _: &TypeVar, _: &<Lang as Language>::Type) -> Self::Target {
        self
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

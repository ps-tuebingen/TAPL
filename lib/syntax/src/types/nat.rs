use super::Type;
use crate::{
    TypeVar,
    free_vars::FreeTypeVars,
    language::Language,
    span::{Span, Spanned},
    subst::SubstType,
};
use macros::EqNoSpan;
use std::{collections::HashSet, fmt, marker::PhantomData};

/// Type of natural numbers
#[derive(Clone, Debug, EqNoSpan)]
pub struct Nat<Lang>
where
    Lang: Language,
{
    /// Source location
    pub span: Span,
    /// Save the type parameter
    phantom: PhantomData<Lang>,
}

impl<Lang> Nat<Lang>
where
    Lang: Language,
{
    /// Create a new Nat from span
    #[must_use]
    pub const fn new(span: Span) -> Self {
        Self {
            span,
            phantom: PhantomData,
        }
    }
}

impl<Lang> Spanned for Nat<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> FreeTypeVars for Nat<Lang>
where
    Lang: Language,
{
    fn free_type_vars(&self, _: &mut HashSet<TypeVar>) {
        ()
    }
}

impl<Lang> Type for Nat<Lang> where Lang: Language {}

impl<Lang> SubstType for Nat<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, _: &TypeVar, _: &<Lang as Language>::Type) -> Self::Target {
        self
    }
}

impl<Lang> fmt::Display for Nat<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Nat")
    }
}

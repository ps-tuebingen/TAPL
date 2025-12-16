use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    span::{Span, Spanned},
    subst::{SubstTerm, SubstType},
};
use macros::EqNoSpan;
use std::{fmt, marker::PhantomData};

/// Term representing `true`
#[derive(Clone, Debug, EqNoSpan)]
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
    /// Create a new true with given span
    #[must_use]
    pub const fn new(span: Span) -> Self {
        Self {
            phantom: PhantomData,
            span,
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

impl<Lang> Term for True<Lang> where Lang: Language {}

impl<Lang> SubstTerm for True<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, _: &Var, _: &<Lang as Language>::Term) -> Self::Target {
        self
    }
}

impl<Lang> SubstType for True<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, _: &TypeVar, _: &<Lang as Language>::Type) -> Self::Target {
        self
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

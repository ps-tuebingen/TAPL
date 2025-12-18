use super::Type;
use crate::{
    TypeVar,
    language::Language,
    span::{Span, Spanned},
    subst::SubstType,
};
use macros::EqNoSpan;
use std::{fmt, marker::PhantomData};

/// Boolean type
#[derive(Clone, Debug, EqNoSpan)]
pub struct Bool<Lang>
where
    Lang: Language,
{
    /// Source location
    pub span: Span,
    /// Save the type parameter
    phantom: PhantomData<Lang>,
}

impl<Lang> Bool<Lang>
where
    Lang: Language,
{
    /// Create a new boolean from span
    #[must_use]
    pub const fn new(span: Span) -> Self {
        Self {
            span,
            phantom: PhantomData,
        }
    }
}

impl<Lang> Spanned for Bool<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Type for Bool<Lang> where Lang: Language {}

impl<Lang> SubstType for Bool<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, _: &TypeVar, _: &<Lang as Language>::Type) -> Self::Target {
        self
    }
}

impl<Lang> fmt::Display for Bool<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Bool")
    }
}

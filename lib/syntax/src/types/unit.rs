use super::Type;
use crate::{
    TypeVar,
    language::Language,
    span::{Span, Spanned},
    subst::SubstType,
};
use macros::EqNoSpan;
use std::{fmt, marker::PhantomData};

/// Unit Type
#[derive(Clone, Debug, EqNoSpan)]
pub struct Unit<Lang>
where
    Lang: Language,
{
    /// Source Location
    pub span: Span,
    /// Save the type parameter
    phantom: PhantomData<Lang>,
}

impl<Lang> Unit<Lang>
where
    Lang: Language,
{
    /// Create a new Unit with given span
    #[must_use]
    pub const fn new(span: Span) -> Self {
        Self {
            span,
            phantom: PhantomData,
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

impl<Lang> Type for Unit<Lang> where Lang: Language {}

impl<Lang> SubstType for Unit<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, _: &TypeVar, _: &<Lang as Language>::Type) -> Self::Target {
        self
    }
}

impl<Lang> fmt::Display for Unit<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Unit")
    }
}

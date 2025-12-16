use super::Type;
use crate::{
    TypeVar,
    kinds::Kind,
    language::Language,
    span::{Span, Spanned},
    subst::SubstType,
};
use std::{fmt, marker::PhantomData};

/// Top Type
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Top<Lang>
where
    Lang: Language,
{
    /// Kind of the type
    pub kind: Kind,
    /// Source Location
    pub span: Span,
    // Save the type Parameter
    phantom: PhantomData<Lang>,
}

impl<Lang> Top<Lang>
where
    Lang: Language,
{
    /// Create new Top Type with kind and span
    #[must_use]
    pub const fn new(knd: Kind, span: Span) -> Self {
        Self {
            kind: knd,
            span,
            phantom: PhantomData,
        }
    }

    /// Create new Top Type with Star kind and given span
    #[must_use]
    pub const fn new_star(span: Span) -> Self {
        Self {
            kind: Kind::Star,
            span,
            phantom: PhantomData,
        }
    }
}

impl<Lang> Spanned for Top<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Type for Top<Lang> where Lang: Language {}

impl<Lang> SubstType for Top<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, _: &TypeVar, _: &<Lang as Language>::Type) -> Self::Target {
        self
    }
}

impl<Lang> fmt::Display for Top<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Top[{}]", self.kind)
    }
}

use crate::{
    TypeVar,
    free_vars::FreeTypeVars,
    kinds::Kind,
    language::Language,
    span::{Span, Spanned},
    subst::SubstType,
    types::Type,
};
use macros::EqNoSpan;
use std::{collections::HashSet, fmt, marker::PhantomData};

/// Bottom Type
#[derive(Debug, Clone, EqNoSpan)]
pub struct Bot<Lang>
where
    Lang: Language,
{
    /// kind of the type
    pub kind: Kind,
    /// Source location
    pub span: Span,
    /// Save the type parameter
    phantom: PhantomData<Lang>,
}

impl<Lang> Bot<Lang>
where
    Lang: Language,
{
    /// Create a new bottom type with kind star from span
    #[must_use]
    pub const fn new(span: Span) -> Self {
        Self {
            kind: Kind::Star,
            span,
            phantom: PhantomData,
        }
    }
}

impl<Lang> Spanned for Bot<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> FreeTypeVars for Bot<Lang>
where
    Lang: Language,
{
    fn free_type_vars(&self, _: &mut HashSet<TypeVar>) {
        ()
    }
}

impl<Lang> Type for Bot<Lang> where Lang: Language {}

impl<Lang> SubstType for Bot<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, _: &TypeVar, _: &<Lang as Language>::Type) -> Self::Target {
        self
    }
}

impl<Lang> fmt::Display for Bot<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Bot")
    }
}

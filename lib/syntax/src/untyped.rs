use crate::{
    TypeVar,
    free_vars::FreeTypeVars,
    language::Language,
    span::{Span, Spanned},
    subst::SubstType,
    types::{Bool, Fun, Nat, Type, TypeGroup},
};
use macros::EqNoSpan;
use std::{collections::HashSet, fmt, marker::PhantomData};

/// "Type" for unpyped languages
#[derive(Clone, Copy, Debug, EqNoSpan)]
pub struct Untyped<Lang>
where
    Lang: Language,
{
    /// Save the type parameter
    phantom: PhantomData<Lang::Term>,
    /// Source location
    pub span: Span,
}

impl<Lang> Untyped<Lang>
where
    Lang: Language,
{
    /// Create a new Untyped at given span
    #[must_use]
    pub const fn new(span: Span) -> Self {
        Self {
            span,
            phantom: PhantomData,
        }
    }
}

impl<Lang> Spanned for Untyped<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}
impl<Lang> FreeTypeVars for Untyped<Lang>
where
    Lang: Language,
{
    fn free_type_vars(&self, _: &mut HashSet<TypeVar>) {
        ()
    }
}
impl<Lang> Type for Untyped<Lang> where Lang: Language {}

impl<Lang> fmt::Display for Untyped<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("")
    }
}

impl<Lang> SubstType for Untyped<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, _: &TypeVar, _: &<Lang as Language>::Type) -> Self::Target {
        self
    }
}

impl<Lang> TypeGroup for Untyped<Lang>
where
    Lang: Language,
{
    type Lang = Lang;
}

impl<Lang> From<Fun<Lang>> for Untyped<Lang>
where
    Lang: Language,
{
    fn from(fun: Fun<Lang>) -> Self {
        Self::new(fun.span)
    }
}

impl<Lang> From<Bool<Lang>> for Untyped<Lang>
where
    Lang: Language,
{
    fn from(b: Bool<Lang>) -> Self {
        Self::new(b.span)
    }
}

impl<Lang> From<Nat<Lang>> for Untyped<Lang>
where
    Lang: Language,
{
    fn from(n: Nat<Lang>) -> Self {
        Self::new(n.span)
    }
}

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

/// Term representing a number
#[derive(Clone, Debug, EqNoSpan)]
pub struct Num<Lang>
where
    Lang: Language,
{
    /// Number
    pub num: i64,
    /// Source location
    pub span: Span,
    /// Save the type parameter
    phantom: PhantomData<Lang>,
}

impl<Lang> Num<Lang>
where
    Lang: Language,
{
    #[must_use]
    /// Create a new Num with given number and span
    pub const fn new(num: i64, span: Span) -> Self {
        Self {
            num,
            span,
            phantom: PhantomData,
        }
    }
}

impl<Lang> Spanned for Num<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> FreeVars for Num<Lang>
where
    Lang: Language,
{
    fn free_vars(&self, _: &mut HashSet<Var>) {}
}

impl<Lang> FreeTypeVars for Num<Lang>
where
    Lang: Language,
{
    fn free_type_vars(&self, _: &mut HashSet<TypeVar>) {}
}

impl<Lang> Term for Num<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Num<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, _: &Var, _: &<Lang as Language>::Term) -> Self::Target {
        self
    }
}

impl<Lang> SubstType for Num<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, _: &TypeVar, _: &<Lang as Language>::Type) -> Self::Target {
        self
    }
}

impl<Lang> fmt::Display for Num<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.num)
    }
}

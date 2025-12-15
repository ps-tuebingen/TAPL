use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    span::{Span, Spanned},
    subst::{SubstTerm, SubstType},
};
use std::fmt;

/// Term representing Nothing/None
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Nothing<Lang>
where
    Lang: Language,
{
    /// Annotated type
    pub ty: Lang::Type,
    /// Source location
    pub span: Span,
}

impl<Lang> Nothing<Lang>
where
    Lang: Language,
{
    /// Create a new Nothing term with given type and span
    pub fn new<Typ>(ty: Typ, span: Span) -> Self
    where
        Typ: Into<Lang::Type>,
    {
        Self {
            ty: ty.into(),
            span,
        }
    }
}

impl<Lang> Spanned for Nothing<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Term for Nothing<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Nothing<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, _: &Var, _: &<Lang as Language>::Term) -> Self::Target {
        self
    }
}

impl<Lang> SubstType for Nothing<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(mut self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        self.ty = self.ty.subst_type(v, ty);
        self
    }
}

impl<Lang> fmt::Display for Nothing<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Nothing[{}]", self.ty)
    }
}

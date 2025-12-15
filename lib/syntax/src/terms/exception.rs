use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    span::{Span, Spanned},
    subst::{SubstTerm, SubstType},
};
use std::fmt;

/// Term representing an exception/error
/// without value
/// used with [`crate::terms::tryt::Try`]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Exception<Lang>
where
    Lang: Language,
{
    /// Annotated type
    pub ty: Lang::Type,
    /// Source location
    pub span: Span,
}

impl<Lang> Exception<Lang>
where
    Lang: Language,
{
    /// Create a new exception term with given type annotation and span
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

impl<Lang> Spanned for Exception<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Term for Exception<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Exception<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, _: &Var, _: &Lang::Term) -> Self::Target {
        self
    }
}

impl<Lang> SubstType for Exception<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Self {
            ty: self.ty.subst_type(v, ty),
            span: self.span,
        }
    }
}

impl<Lang> fmt::Display for Exception<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error[{}]", self.ty)
    }
}

use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    span::{Span, Spanned},
    subst::{SubstTerm, SubstType},
};
use macros::EqNoSpan;
use std::{fmt, rc::Rc};

/// Term representing a fold term
/// `fold[ty] t`
#[derive(Clone, Debug, EqNoSpan)]
pub struct Fold<Lang>
where
    Lang: Language,
{
    /// Inner term
    pub term: Rc<Lang::Term>,
    /// Type to fold
    pub ty: Lang::Type,
    /// Source location
    pub span: Span,
}

impl<Lang> Fold<Lang>
where
    Lang: Language,
{
    /// Create a new Fold term with given inner term, type and span
    pub fn new<T1, Typ>(t: T1, ty: Typ, span: Span) -> Self
    where
        T1: Into<Lang::Term>,
        Typ: Into<Lang::Type>,
    {
        Self {
            term: Rc::new(t.into()),
            ty: ty.into(),
            span,
        }
    }
}

impl<Lang> Spanned for Fold<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Term for Fold<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Fold<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        Self {
            term: self.term.subst(v, t),
            ty: self.ty,
            span: self.span,
        }
    }
}

impl<Lang> SubstType for Fold<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Self {
            term: self.term.subst_type(v, ty),
            ty: self.ty.subst_type(v, ty),
            span: self.span,
        }
    }
}

impl<Lang> fmt::Display for Fold<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fold[{}]({})", self.ty, self.term)
    }
}

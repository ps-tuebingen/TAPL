use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    span::{Span, Spanned},
    subst::{SubstTerm, SubstType},
};
use macros::EqNoSpan;
use std::{fmt, rc::Rc};

/// Term representing unfolding a type
#[derive(Clone, Debug, EqNoSpan)]
pub struct Unfold<Lang>
where
    Lang: Language,
{
    /// Type to unfold
    pub ty: Lang::Type,
    /// Inner term
    pub term: Rc<Lang::Term>,
    /// Source location
    pub span: Span,
}

impl<Lang> Unfold<Lang>
where
    Lang: Language,
{
    /// Create a new unfold term with given type, inner term and span
    pub fn new<T1, Ty1>(ty: Ty1, t: T1, span: Span) -> Self
    where
        T1: Into<Lang::Term>,
        Ty1: Into<Lang::Type>,
    {
        Self {
            ty: ty.into(),
            term: Rc::new(t.into()),
            span,
        }
    }
}

impl<Lang> Spanned for Unfold<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Term for Unfold<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Unfold<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(mut self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        self.term = self.term.subst(v, t);
        self
    }
}

impl<Lang> SubstType for Unfold<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(mut self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        self.ty = self.ty.subst_type(v, ty);
        self.term = self.term.subst_type(v, ty);
        self
    }
}

impl<Lang> fmt::Display for Unfold<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unfold[{}]({})", self.ty, self.term)
    }
}

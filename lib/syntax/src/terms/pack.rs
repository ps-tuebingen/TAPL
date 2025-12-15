use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    span::{Span, Spanned},
    subst::{SubstTerm, SubstType},
};
use std::{fmt, rc::Rc};

/// Term representing packing an existential type
/// `{*ty1,t} as ty2`
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pack<Lang>
where
    Lang: Language,
{
    /// Inner type
    pub inner_ty: Lang::Type,
    /// Inner term
    pub term: Rc<Lang::Term>,
    /// Outer type (existential)
    pub outer_ty: Lang::Type,
    /// Source location
    pub span: Span,
}

impl<Lang> Pack<Lang>
where
    Lang: Language,
{
    /// Create a new pack with given inner type, inner term, outer type and span
    pub fn new<Ty1, Ty2, T1>(inner: Ty1, t: T1, outer: Ty2, span: Span) -> Self
    where
        Ty1: Into<Lang::Type>,
        Ty2: Into<Lang::Type>,
        T1: Into<Lang::Term>,
    {
        Self {
            inner_ty: inner.into(),
            term: Rc::new(t.into()),
            outer_ty: outer.into(),
            span,
        }
    }
}

impl<Lang> Spanned for Pack<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Term for Pack<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Pack<Lang>
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

impl<Lang> SubstType for Pack<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(mut self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        self.inner_ty = self.inner_ty.subst_type(v, ty);
        self.term = self.term.subst_type(v, ty);
        self.outer_ty = self.outer_ty.subst_type(v, ty);
        self
    }
}

impl<Lang> fmt::Display for Pack<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{*({}),{}}} as {}",
            self.inner_ty, self.term, self.outer_ty
        )
    }
}

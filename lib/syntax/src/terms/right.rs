use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    span::{Span, Spanned},
    subst::{SubstTerm, SubstType},
};
use macros::{EqNoSpan, HashNoSpan};
use std::{fmt, rc::Rc};

/// Term representing a right injection into a sum
#[derive(HashNoSpan, Clone, Debug, EqNoSpan)]
pub struct Right<Lang>
where
    Lang: Language,
{
    /// Term to inject
    pub right_term: Rc<Lang::Term>,
    /// Annotated sum type
    pub ty: Lang::Type,
    /// Source location
    pub span: Span,
}

impl<Lang> Right<Lang>
where
    Lang: Language,
{
    /// Create a new right with given term, type and span
    pub fn new<T1, Ty1>(right_t: T1, ty: Ty1, span: Span) -> Self
    where
        T1: Into<Lang::Term>,
        Ty1: Into<Lang::Type>,
    {
        Self {
            right_term: Rc::new(right_t.into()),
            ty: ty.into(),
            span,
        }
    }
}

impl<Lang> Spanned for Right<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Term for Right<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Right<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(mut self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        self.right_term = self.right_term.subst(v, t);
        self
    }
}

impl<Lang> SubstType for Right<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(mut self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        self.right_term = self.right_term.subst_type(v, ty);
        self.ty = self.ty.subst_type(v, ty);
        self
    }
}

impl<Lang> fmt::Display for Right<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "inl({}) as {}", self.right_term, self.ty)
    }
}

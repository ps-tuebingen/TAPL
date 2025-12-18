use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    span::{Span, Spanned},
    subst::{SubstTerm, SubstType},
};
use macros::{EqNoSpan, HashNoSpan};
use std::{fmt, rc::Rc};

/// Term representing left injection into a sum
#[derive(HashNoSpan, Clone, Debug, EqNoSpan)]
pub struct Left<Lang>
where
    Lang: Language,
{
    /// inner term
    pub left_term: Rc<Lang::Term>,
    /// annotated sum type
    pub ty: Lang::Type,
    /// Source location
    pub span: Span,
}

impl<Lang> Left<Lang>
where
    Lang: Language,
{
    /// Create a new left term with given term, type and span
    pub fn new<L, Typ>(left_t: L, ty: Typ, span: Span) -> Self
    where
        L: Into<Lang::Term>,
        Typ: Into<Lang::Type>,
    {
        Self {
            left_term: Rc::new(left_t.into()),
            ty: ty.into(),
            span,
        }
    }
}

impl<Lang> Spanned for Left<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Term for Left<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Left<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        Self {
            left_term: self.left_term.subst(v, t),
            ty: self.ty,
            span: self.span,
        }
    }
}

impl<Lang> SubstType for Left<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Self {
            left_term: self.left_term.subst_type(v, ty),
            ty: self.ty.subst_type(v, ty),
            span: self.span,
        }
    }
}

impl<Lang> fmt::Display for Left<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "inl({}) as {}", self.left_term, self.ty)
    }
}

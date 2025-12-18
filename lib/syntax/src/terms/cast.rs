use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    span::{Span, Spanned},
    subst::{SubstTerm, SubstType},
};
use macros::{EqNoSpan, HashNoSpan};
use std::{fmt, rc::Rc};

/// Term representing a cast `t as ty`
#[derive(HashNoSpan, Clone, Debug, EqNoSpan)]
pub struct Cast<Lang>
where
    Lang: Language,
{
    /// The casted term
    pub term: Rc<Lang::Term>,
    /// The type to cast to
    pub ty: Lang::Type,
    /// The source location
    pub span: Span,
}

impl<Lang> Cast<Lang>
where
    Lang: Language,
{
    /// Create a new cast from a given term, type and span
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

impl<Lang> Spanned for Cast<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Term for Cast<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Cast<Lang>
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

impl<Lang> SubstType for Cast<Lang>
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

impl<Lang> fmt::Display for Cast<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} as {} ", self.term, self.ty)
    }
}

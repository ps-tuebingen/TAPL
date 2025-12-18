use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    span::{Span, Spanned},
    subst::{SubstTerm, SubstType},
};
use macros::EqNoSpan;
use std::{fmt, rc::Rc};

/// Term representing getting the tail of a list
#[derive(Clone, Debug, EqNoSpan)]
pub struct Tail<Lang>
where
    Lang: Language,
{
    /// The list term
    pub term: Rc<Lang::Term>,
    /// Annotated type
    pub ty: Lang::Type,
    /// Source Location
    pub span: Span,
}

impl<Lang> Tail<Lang>
where
    Lang: Language,
{
    /// Create a new tail term from inner term, type and span
    pub fn new<T1, Ty1>(t: T1, ty: Ty1, span: Span) -> Self
    where
        T1: Into<Lang::Term>,
        Ty1: Into<Lang::Type>,
    {
        Self {
            term: Rc::new(t.into()),
            ty: ty.into(),
            span,
        }
    }
}

impl<Lang> Spanned for Tail<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Term for Tail<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Tail<Lang>
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

impl<Lang> SubstType for Tail<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(mut self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        self.term = self.term.subst_type(v, ty);
        self.ty = self.ty.subst_type(v, ty);
        self
    }
}

impl<Lang> fmt::Display for Tail<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "tail[{}]({})", self.term, self.term)
    }
}

use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    span::{Span, Spanned},
    subst::{SubstTerm, SubstType},
};
use macros::{EqNoSpan, HashNoSpan};
use std::{fmt, rc::Rc};

/// Term representing a tuple projection
#[derive(HashNoSpan, Clone, Debug, EqNoSpan)]
pub struct Projection<Lang>
where
    Lang: Language,
{
    /// Tuple to project
    pub term: Rc<Lang::Term>,
    /// Projection index
    pub index: usize,
    /// Source location
    pub span: Span,
}

impl<Lang> Projection<Lang>
where
    Lang: Language,
{
    /// Crate a new projection from a given term, index and span
    pub fn new<T1>(t: T1, ind: usize, span: Span) -> Self
    where
        T1: Into<Lang::Term>,
    {
        Self {
            term: Rc::new(t.into()),
            index: ind,
            span,
        }
    }
}

impl<Lang> Spanned for Projection<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Term for Projection<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Projection<Lang>
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

impl<Lang> SubstType for Projection<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(mut self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        self.term = self.term.subst_type(v, ty);
        self
    }
}

impl<Lang> fmt::Display for Projection<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}).{}", self.term, self.index)
    }
}

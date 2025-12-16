use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    span::{Span, Spanned},
    subst::{SubstTerm, SubstType},
};
use macros::EqNoSpan;
use std::{fmt, rc::Rc};

/// Term representing the successor of a number
#[derive(Clone, Debug, EqNoSpan)]
pub struct Succ<Lang>
where
    Lang: Language,
{
    /// Argument
    pub term: Rc<Lang::Term>,
    /// Source location
    pub span: Span,
}

impl<Lang> Succ<Lang>
where
    Lang: Language,
{
    /// Create a new succ from given term and span
    pub fn new<T1>(t: T1, span: Span) -> Self
    where
        T1: Into<Lang::Term>,
    {
        Self {
            term: Rc::new(t.into()),
            span,
        }
    }
}

impl<Lang> Spanned for Succ<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Term for Succ<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Succ<Lang>
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

impl<Lang> SubstType for Succ<Lang>
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

impl<Lang> fmt::Display for Succ<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "succ({})", self.term)
    }
}

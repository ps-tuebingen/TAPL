use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    span::{Span, Spanned},
    subst::{SubstTerm, SubstType},
};
use macros::{EqNoSpan, HashNoSpan};
use std::{fmt, rc::Rc};

/// Term representing try-raise
/// used with [`crate::terms::raise::Raise`]
#[derive(HashNoSpan, Clone, Debug, EqNoSpan)]
pub struct TryWithVal<Lang>
where
    Lang: Language,
{
    /// Term that might raise an exception
    pub term: Rc<Lang::Term>,
    /// Exception handler
    pub handler: Rc<Lang::Term>,
    /// Source location
    pub span: Span,
}

impl<Lang> TryWithVal<Lang>
where
    Lang: Language,
{
    /// Create a new tryval with given inner term, handler and span
    pub fn new<T1, T2>(t: T1, h: T2, span: Span) -> Self
    where
        T1: Into<Lang::Term>,
        T2: Into<Lang::Term>,
    {
        Self {
            term: Rc::new(t.into()),
            handler: Rc::new(h.into()),
            span,
        }
    }
}

impl<Lang> Spanned for TryWithVal<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Term for TryWithVal<Lang> where Lang: Language {}

impl<Lang> SubstTerm for TryWithVal<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(mut self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        self.term = self.term.subst(v, t);
        self.handler = self.handler.subst(v, t);
        self
    }
}

impl<Lang> SubstType for TryWithVal<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(mut self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        self.term = self.term.subst_type(v, ty);
        self.handler = self.handler.subst_type(v, ty);
        self
    }
}

impl<Lang> fmt::Display for TryWithVal<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "try {{ {} }} catch {{ {} }}", self.term, self.handler)
    }
}

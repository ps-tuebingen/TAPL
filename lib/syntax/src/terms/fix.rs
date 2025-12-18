use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    span::{Span, Spanned},
    subst::{SubstTerm, SubstType},
};
use macros::EqNoSpan;
use std::{fmt, rc::Rc};

/// Term representing a fixed point combinator `fix(t)`
#[derive(Clone, Debug, EqNoSpan)]
pub struct Fix<Lang>
where
    Lang: Language,
{
    /// The term to fix
    pub term: Rc<Lang::Term>,
    /// Source location
    pub span: Span,
}

impl<Lang> Fix<Lang>
where
    Lang: Language,
{
    /// Create a new fix term with given inner term and span
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

impl<Lang> Spanned for Fix<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Term for Fix<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Fix<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        Self {
            term: self.term.subst(v, t),
            span: self.span,
        }
    }
}

impl<Lang> SubstType for Fix<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Self {
            term: self.term.subst_type(v, ty),
            span: self.span,
        }
    }
}

impl<Lang> fmt::Display for Fix<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fix({})", self.term)
    }
}

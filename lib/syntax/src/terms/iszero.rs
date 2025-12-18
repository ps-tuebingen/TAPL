use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    span::{Span, Spanned},
    subst::{SubstTerm, SubstType},
};
use macros::{EqNoSpan, HashNoSpan};
use std::{fmt, rc::Rc};

/// Term representing calling `iszero` on a number
#[derive(HashNoSpan, Clone, Debug, EqNoSpan)]
pub struct IsZero<Lang>
where
    Lang: Language,
{
    /// The number
    pub term: Rc<Lang::Term>,
    /// Source location
    pub span: Span,
}

impl<Lang> IsZero<Lang>
where
    Lang: Language,
{
    /// Create a new iszero term with given inner term and span
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

impl<Lang> Spanned for IsZero<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Term for IsZero<Lang> where Lang: Language {}

impl<Lang> SubstTerm for IsZero<Lang>
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

impl<Lang> SubstType for IsZero<Lang>
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

impl<Lang> fmt::Display for IsZero<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "iszero({})", self.term)
    }
}

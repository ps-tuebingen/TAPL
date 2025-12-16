use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    span::{Span, Spanned},
    subst::{SubstTerm, SubstType},
};
use macros::EqNoSpan;
use std::{fmt, rc::Rc};

/// Term representing a memory assignment
#[derive(Clone, Debug, EqNoSpan)]
pub struct Assign<Lang>
where
    Lang: Language,
{
    /// Left hand side of the assignment (memory location)
    pub lhs: Rc<Lang::Term>,
    /// Right hand side of the assignment (value)
    pub rhs: Rc<Lang::Term>,
    /// Source Span
    pub span: Span,
}

impl<Lang> Assign<Lang>
where
    Lang: Language,
{
    /// Create a new assign term from given left and right-hand sides
    pub fn new<T1, T2>(lhs: T1, rhs: T2) -> Self
    where
        T1: Spanned + Into<Lang::Term>,
        T2: Spanned + Into<Lang::Term>,
    {
        let span = lhs.span().extend(&rhs.span());
        Self {
            lhs: Rc::new(lhs.into()),
            rhs: Rc::new(rhs.into()),
            span,
        }
    }
}

impl<Lang> Spanned for Assign<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Term for Assign<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Assign<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        Self {
            lhs: self.lhs.subst(v, t),
            rhs: self.rhs.subst(v, t),
            span: self.span,
        }
    }
}

impl<Lang> SubstType for Assign<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Self {
            lhs: self.lhs.subst_type(v, ty),
            rhs: self.rhs.subst_type(v, ty),
            span: self.span,
        }
    }
}

impl<Lang> fmt::Display for Assign<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}) := {}", self.lhs, self.rhs)
    }
}

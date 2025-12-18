use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    span::{Span, Spanned},
    subst::{SubstTerm, SubstType},
};
use macros::{EqNoSpan, HashNoSpan};
use std::{fmt, rc::Rc};

/// Term representing a case on an option
#[derive(HashNoSpan, Clone, Debug, EqNoSpan)]
pub struct SomeCase<Lang>
where
    Lang: Language,
{
    /// Term to match against
    pub bound_term: Rc<Lang::Term>,
    /// None case
    pub none_term: Rc<Lang::Term>,
    /// Bound variable for Some case
    pub some_var: Var,
    /// Some case (with [`Self::some_var`] in scope)
    pub some_term: Rc<Lang::Term>,
    /// Source location
    pub span: Span,
}

impl<Lang> SomeCase<Lang>
where
    Lang: Language,
{
    /// Create a new some case with bound term, none rhs, bound some variable, some rhs and span
    pub fn new<T1, T2, T3>(bound: T1, none: T2, v: &str, some: T3, span: Span) -> Self
    where
        T1: Into<Lang::Term>,
        T2: Into<Lang::Term>,
        T3: Into<Lang::Term>,
    {
        Self {
            bound_term: Rc::new(bound.into()),
            none_term: Rc::new(none.into()),
            some_var: v.to_owned(),
            some_term: Rc::new(some.into()),
            span,
        }
    }
}

impl<Lang> Spanned for SomeCase<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Term for SomeCase<Lang> where Lang: Language {}

impl<Lang> SubstTerm for SomeCase<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(mut self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        self.bound_term = self.bound_term.subst(v, t);
        self.none_term = self.none_term.subst(v, t);
        if *v != self.some_var {
            self.some_term = self.some_term.subst(v, t);
        }
        self
    }
}

impl<Lang> SubstType for SomeCase<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(mut self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        self.bound_term = self.bound_term.subst_type(v, ty);
        self.none_term = self.none_term.subst_type(v, ty);
        self.some_term = self.some_term.subst_type(v, ty);
        self
    }
}

impl<Lang> fmt::Display for SomeCase<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "case {} of {{ Nothing => {} | Something({}) => {} }}",
            self.bound_term, self.none_term, self.some_var, self.some_term
        )
    }
}

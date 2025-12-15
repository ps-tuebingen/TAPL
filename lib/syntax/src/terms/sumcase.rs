use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    span::{Span, Spanned},
    subst::{SubstTerm, SubstType},
};
use std::{fmt, rc::Rc};

/// Term representing a case for a sum type
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SumCase<Lang>
where
    Lang: Language,
{
    /// Term to match against
    pub bound_term: Rc<Lang::Term>,
    /// Bound variable for the left case
    pub left_var: Var,
    /// Left case (with [`Self::left_var`] in scope)
    pub left_term: Rc<Lang::Term>,
    /// Bound variable for the right case
    pub right_var: Var,
    /// Right case (with [`Self::right_var`] in scope)
    pub right_term: Rc<Lang::Term>,
    /// Source location
    pub span: Span,
}

impl<Lang> SumCase<Lang>
where
    Lang: Language,
{
    /// Create a new sum case with given bound term, left variable, left term, right variable,
    /// right term and span
    pub fn new<T1, T2, T3>(
        bound: T1,
        left_v: &str,
        left_t: T2,
        right_v: &str,
        right_t: T3,
        span: Span,
    ) -> Self
    where
        T1: Into<Lang::Term>,
        T2: Into<Lang::Term>,
        T3: Into<Lang::Term>,
    {
        Self {
            bound_term: Rc::new(bound.into()),
            left_var: left_v.to_owned(),
            left_term: Rc::new(left_t.into()),
            right_var: right_v.to_owned(),
            right_term: Rc::new(right_t.into()),
            span,
        }
    }
}

impl<Lang> Spanned for SumCase<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Term for SumCase<Lang> where Lang: Language {}

impl<Lang> SubstTerm for SumCase<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(mut self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        self.bound_term = self.bound_term.subst(v, t);
        if *v != self.left_var {
            self.left_term = self.left_term.subst(v, t);
        }
        if *v != self.right_var {
            self.right_term = self.right_term.subst(v, t);
        }
        self
    }
}

impl<Lang> SubstType for SumCase<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(mut self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        self.bound_term = self.bound_term.subst_type(v, ty);
        self.left_term = self.left_term.subst_type(v, ty);
        self.right_term = self.right_term.subst_type(v, ty);
        self
    }
}

impl<Lang> fmt::Display for SumCase<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "case {} of {{ inl({}) => {} | inr({}) => {} }}",
            self.bound_term, self.left_var, self.left_term, self.right_var, self.right_term
        )
    }
}

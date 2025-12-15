use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    span::{Span, Spanned},
    subst::{SubstTerm, SubstType},
};
use std::{fmt, rc::Rc};

/// Term representing a let binding
/// `let x = t1 in t2`
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Let<Lang>
where
    Lang: Language,
{
    /// Bound variable
    pub var: Var,
    /// Bound Term
    pub bound_term: Rc<Lang::Term>,
    /// Next term
    pub in_term: Rc<Lang::Term>,
    /// Souce location
    pub span: Span,
}

impl<Lang> Let<Lang>
where
    Lang: Language,
{
    /// Create a new let binding with given variable, bound term, in term and span
    pub fn new<T1, T2>(v: &str, bound: T1, int: T2, span: Span) -> Self
    where
        T1: Into<Lang::Term>,
        T2: Into<Lang::Term>,
    {
        Self {
            var: v.to_owned(),
            bound_term: Rc::new(bound.into()),
            in_term: Rc::new(int.into()),
            span,
        }
    }
}

impl<Lang> Spanned for Let<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Term for Let<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Let<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        if *v == self.var {
            self
        } else {
            Self {
                var: self.var,
                bound_term: self.bound_term.subst(v, t),
                in_term: self.in_term.subst(v, t),
                span: self.span,
            }
        }
    }
}

impl<Lang> SubstType for Let<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Self {
            var: self.var,
            bound_term: self.bound_term.subst_type(v, ty),
            in_term: self.in_term.subst_type(v, ty),
            span: self.span,
        }
    }
}

impl<Lang> fmt::Display for Let<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "let ({} = {}) in {}",
            self.var, self.bound_term, self.in_term
        )
    }
}

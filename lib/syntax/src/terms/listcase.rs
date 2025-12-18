use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    span::{Span, Spanned},
    subst::{SubstTerm, SubstType},
};
use macros::EqNoSpan;
use std::{fmt, rc::Rc};

/// Term representing a list case
/// `case t1 of { Nil => t2, Cons(x,xs) => t3 }`
#[derive(Clone, Debug, EqNoSpan)]
pub struct ListCase<Lang>
where
    Lang: Language,
{
    /// Term to match
    pub bound_term: Rc<Lang::Term>,
    /// Nil case
    pub nil_rhs: Rc<Lang::Term>,
    /// Bound variable for the list head
    pub cons_fst: Var,
    /// Bound variable for the list tail
    pub cons_rst: Var,
    /// Cons Case (with [`Self::cons_fst`] and [`Self::cons_rst`] in scope
    pub cons_rhs: Rc<Lang::Term>,
    /// Source location
    pub span: Span,
}

impl<Lang> ListCase<Lang>
where
    Lang: Language,
{
    /// Create a new list case with bound term, nil case, head and tail variables, cons case and
    /// span
    pub fn new<T1, T2, T3>(bound: T1, nil: T2, hd: &str, tl: &str, cons: T3, span: Span) -> Self
    where
        T1: Into<Lang::Term>,
        T2: Into<Lang::Term>,
        T3: Into<Lang::Term>,
    {
        Self {
            bound_term: Rc::new(bound.into()),
            nil_rhs: Rc::new(nil.into()),
            cons_fst: hd.to_owned(),
            cons_rst: tl.to_owned(),
            cons_rhs: Rc::new(cons.into()),
            span,
        }
    }
}

impl<Lang> Spanned for ListCase<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Term for ListCase<Lang> where Lang: Language {}

impl<Lang> SubstTerm for ListCase<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(mut self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        self.bound_term = self.bound_term.subst(v, t);
        self.nil_rhs = self.nil_rhs.subst(v, t);
        if *v != self.cons_fst && *v != self.cons_rst {
            self.cons_rhs = self.cons_rhs.subst(v, t);
        }
        self
    }
}

impl<Lang> SubstType for ListCase<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Self {
            bound_term: self.bound_term.subst_type(v, ty),
            nil_rhs: self.nil_rhs.subst_type(v, ty),
            cons_fst: self.cons_fst,
            cons_rst: self.cons_rst,
            cons_rhs: self.cons_rhs.subst_type(v, ty),
            span: self.span,
        }
    }
}

impl<Lang> fmt::Display for ListCase<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "case {} of {{ Nil => {} | Cons({},{}) => {} }}",
            self.bound_term, self.nil_rhs, self.cons_fst, self.cons_rst, self.cons_rhs
        )
    }
}

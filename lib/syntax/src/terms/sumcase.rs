use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::{fmt, rc::Rc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SumCase<Lang>
where
    Lang: Language,
{
    pub bound_term: Rc<Lang::Term>,
    pub left_var: Var,
    pub left_term: Rc<Lang::Term>,
    pub right_var: Var,
    pub right_term: Rc<Lang::Term>,
}

impl<Lang> SumCase<Lang>
where
    Lang: Language,
{
    pub fn new<T1, T2, T3>(
        bound: T1,
        left_v: &str,
        left_t: T2,
        right_v: &str,
        right_t: T3,
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
        }
    }
}

impl<Lang> Term for SumCase<Lang> where Lang: Language {}

impl<Lang> SubstTerm for SumCase<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        let bound_subst = self.bound_term.subst(v, t);
        let left_term = if *v == self.left_var {
            self.left_term
        } else {
            self.left_term.subst(v, t)
        };
        let right_term = if *v == self.right_var {
            self.right_term
        } else {
            self.right_term.subst(v, t)
        };
        Self {
            bound_term: bound_subst,
            left_var: self.left_var,
            left_term,
            right_var: self.right_var,
            right_term,
        }
    }
}

impl<Lang> SubstType for SumCase<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Self {
            bound_term: self.bound_term.subst_type(v, ty),
            left_var: self.left_var,
            left_term: self.left_term.subst_type(v, ty),
            right_var: self.right_var,
            right_term: self.right_term.subst_type(v, ty),
        }
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

use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SumCase<Lang>
where
    Lang: Language,
{
    pub bound_term: Box<Lang::Term>,
    pub left_var: Var,
    pub left_term: Box<Lang::Term>,
    pub right_var: Var,
    pub right_term: Box<Lang::Term>,
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
    ) -> SumCase<Lang>
    where
        T1: Into<Lang::Term>,
        T2: Into<Lang::Term>,
        T3: Into<Lang::Term>,
    {
        SumCase {
            bound_term: Box::new(bound.into()),
            left_var: left_v.to_owned(),
            left_term: Box::new(left_t.into()),
            right_var: right_v.to_owned(),
            right_term: Box::new(right_t.into()),
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
            Box::new(self.left_term.subst(v, t))
        };
        let right_term = if *v == self.right_var {
            self.right_term
        } else {
            Box::new(self.right_term.subst(v, t))
        };
        SumCase {
            bound_term: Box::new(bound_subst),
            left_var: self.left_var,
            left_term,
            right_var: self.right_var,
            right_term,
        }
        .into()
    }
}

impl<Lang> SubstType for SumCase<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        SumCase {
            bound_term: Box::new(self.bound_term.subst_type(v, ty)),
            left_var: self.left_var,
            left_term: Box::new(self.left_term.subst_type(v, ty)),
            right_var: self.right_var,
            right_term: Box::new(self.right_term.subst_type(v, ty)),
        }
        .into()
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

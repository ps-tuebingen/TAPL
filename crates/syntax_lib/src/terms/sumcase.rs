use super::Term;
use crate::{
    TypeVar, Var,
    subst::{SubstTerm, SubstType},
    types::Type,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SumCase<T>
where
    T: Term,
{
    pub bound_term: Box<T>,
    pub left_var: Var,
    pub left_term: Box<T>,
    pub right_var: Var,
    pub right_term: Box<T>,
}

impl<T> SumCase<T>
where
    T: Term,
{
    pub fn new<T1, T2, T3>(
        bound: T1,
        left_v: &str,
        left_t: T2,
        right_v: &str,
        right_t: T3,
    ) -> SumCase<T>
    where
        T1: Into<T>,
        T2: Into<T>,
        T3: Into<T>,
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

impl<T> Term for SumCase<T> where T: Term {}

impl<T> SubstTerm<T> for SumCase<T>
where
    T: Term + SubstTerm<T, Target = T>,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
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

impl<T, Ty> SubstType<Ty> for SumCase<T>
where
    T: Term + SubstType<Ty, Target = T>,
    Ty: Type,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
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

impl<T> fmt::Display for SumCase<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "case {} of {{ inl({}) => {} | inr({}) => {} }}",
            self.bound_term, self.left_var, self.left_term, self.right_var, self.right_term
        )
    }
}

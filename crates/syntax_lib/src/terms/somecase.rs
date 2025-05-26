use super::Term;
use crate::{
    subst::{SubstTerm, SubstType},
    types::Type,
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SomeCase<T>
where
    T: Term,
{
    pub bound_term: Box<T>,
    pub none_term: Box<T>,
    pub some_var: Var,
    pub some_term: Box<T>,
}

impl<T> SomeCase<T>
where
    T: Term,
{
    pub fn new<T1, T2, T3>(bound: T1, none: T2, v: &str, some: T3) -> SomeCase<T>
    where
        T1: Into<T>,
        T2: Into<T>,
        T3: Into<T>,
    {
        SomeCase {
            bound_term: Box::new(bound.into()),
            none_term: Box::new(none.into()),
            some_var: v.to_owned(),
            some_term: Box::new(some.into()),
        }
    }
}

impl<T> Term for SomeCase<T> where T: Term {}

impl<T> SubstTerm<T> for SomeCase<T>
where
    T: Term + SubstTerm<T, Target = T>,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        if *v == self.some_var {
            SomeCase {
                bound_term: Box::new(self.bound_term.subst(v, t)),
                none_term: Box::new(self.none_term.subst(v, t)),
                some_var: self.some_var,
                some_term: self.some_term,
            }
            .into()
        } else {
            SomeCase {
                bound_term: Box::new(self.bound_term.subst(v, t)),
                none_term: Box::new(self.none_term.subst(v, t)),
                some_var: self.some_var,
                some_term: Box::new(self.some_term.subst(v, t)),
            }
            .into()
        }
    }
}

impl<T, Ty> SubstType<Ty> for SomeCase<T>
where
    T: Term + SubstType<Ty, Target = T>,
    Ty: Type,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        SomeCase {
            bound_term: Box::new(self.bound_term.subst_type(v, ty)),
            none_term: Box::new(self.none_term.subst_type(v, ty)),
            some_var: self.some_var,
            some_term: Box::new(self.some_term.subst_type(v, ty)),
        }
        .into()
    }
}

impl<T> fmt::Display for SomeCase<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "case {} of {{ Nothing => {} | Something({}) => {} }}",
            self.bound_term, self.none_term, self.some_var, self.some_term
        )
    }
}

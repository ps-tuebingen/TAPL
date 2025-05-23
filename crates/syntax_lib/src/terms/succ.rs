use super::Term;
use crate::{
    subst::{SubstTerm, SubstType},
    types::Nat,
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Succ<T>
where
    T: Term,
{
    term: Box<T>,
}

impl<T> Succ<T>
where
    T: Term,
{
    pub fn new<T1>(t: T1) -> Succ<T>
    where
        T1: Into<T>,
    {
        Succ {
            term: Box::new(t.into()),
        }
    }
}

impl<T> Term for Succ<T> where T: Term {}

impl<T> SubstTerm<T> for Succ<T>
where
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Succ {
            term: Box::new(self.term.subst(v, t)),
        }
        .into()
    }
}

impl<T> SubstType<<T as Term>::Type> for Succ<T>
where
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as Term>::Type) -> Self::Target {
        Succ {
            term: Box::new(self.term.subst_type(v, ty)),
        }
        .into()
    }
}

impl<T> fmt::Display for Succ<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "succ({})", self.term)
    }
}

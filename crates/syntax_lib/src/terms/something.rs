use super::Term;
use crate::{
    TypeVar, Var,
    subst::{SubstTerm, SubstType},
    types::Type,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Something<T>
where
    T: Term,
{
    pub term: Box<T>,
}

impl<T> Something<T>
where
    T: Term,
{
    pub fn new<T1>(t: T1) -> Something<T>
    where
        T1: Into<T>,
    {
        Something {
            term: Box::new(t.into()),
        }
    }
}

impl<T> Term for Something<T> where T: Term {}

impl<T> SubstTerm<T> for Something<T>
where
    T: Term + SubstTerm<T, Target = T>,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Something {
            term: Box::new(self.term.subst(v, t)),
        }
        .into()
    }
}

impl<T, Ty> SubstType<Ty> for Something<T>
where
    T: Term + SubstType<Ty, Target = T>,
    Ty: Type,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Something {
            term: Box::new(self.term.subst_type(v, ty)),
        }
        .into()
    }
}

impl<T> fmt::Display for Something<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "something({})", self.term)
    }
}

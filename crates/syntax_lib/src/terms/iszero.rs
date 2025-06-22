use super::Term;
use crate::{
    TypeVar, Var,
    subst::{SubstTerm, SubstType},
    types::Type,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IsZero<T>
where
    T: Term,
{
    pub term: Box<T>,
}

impl<T> IsZero<T>
where
    T: Term,
{
    pub fn new<T1>(t: T1) -> IsZero<T>
    where
        T1: Into<T>,
    {
        IsZero {
            term: Box::new(t.into()),
        }
    }
}

impl<T> Term for IsZero<T> where T: Term {}

impl<T> SubstTerm<T> for IsZero<T>
where
    T: Term + SubstTerm<T, Target = T>,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        IsZero {
            term: Box::new(self.term.subst(v, t)),
        }
        .into()
    }
}

impl<T, Ty> SubstType<Ty> for IsZero<T>
where
    T: Term + SubstType<Ty, Target = T>,
    Ty: Type,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        IsZero {
            term: Box::new(self.term.subst_type(v, ty)),
        }
        .into()
    }
}

impl<T> fmt::Display for IsZero<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "iszero({})", self.term)
    }
}

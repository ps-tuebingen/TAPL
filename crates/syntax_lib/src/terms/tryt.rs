use super::Term;
use crate::{
    subst::{SubstTerm, SubstType},
    types::Type,
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Try<T>
where
    T: Term,
{
    pub term: Box<T>,
    pub handler: Box<T>,
}

impl<T> Try<T>
where
    T: Term,
{
    pub fn new<T1: Into<T>, T2: Into<T>>(t: T1, h: T2) -> Try<T> {
        Try {
            term: Box::new(t.into()),
            handler: Box::new(h.into()),
        }
    }
}

impl<T> Term for Try<T> where T: Term {}

impl<T> SubstTerm<T> for Try<T>
where
    T: Term + SubstTerm<T, Target = T>,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Try {
            term: Box::new(self.term.subst(v, t)),
            handler: Box::new(self.handler.subst(v, t)),
        }
        .into()
    }
}

impl<T, Ty> SubstType<Ty> for Try<T>
where
    T: Term + SubstType<Ty, Target = T>,
    Ty: Type,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Try {
            term: Box::new(self.term.subst_type(v, ty)),
            handler: Box::new(self.handler.subst_type(v, ty)),
        }
        .into()
    }
}

impl<T> fmt::Display for Try<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "try {{ {} }} with {{ {} }}", self.term, self.handler)
    }
}

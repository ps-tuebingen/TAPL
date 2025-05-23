use super::Term;
use crate::{types::Type,
    subst::{SubstTerm, SubstType},
    terms::App,
    types::Fun,
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TryWithVal<T>
where
    T: Term,
{
    term: Box<T>,
    handler: Box<T>,
}

impl<T> TryWithVal<T>
where
    T: Term,
{
    pub fn new<T1, T2>(t: T1, h: T2) -> TryWithVal<T>
    where
        T1: Into<T>,
        T2: Into<T>,
    {
        TryWithVal {
            term: Box::new(t.into()),
            handler: Box::new(h.into()),
        }
    }
}

impl<T> Term for TryWithVal<T> where T: Term {}

impl<T> SubstTerm<T> for TryWithVal<T>
where
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        TryWithVal {
            term: Box::new(self.term.subst(v, t)),
            handler: Box::new(self.handler.subst(v, t)),
        }
        .into()
    }
}

impl<T,Ty> SubstType<Ty> for TryWithVal<T>
where
    T: Term,Ty:Type
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as Term>::Type) -> Self::Target {
        TryWithVal {
            term: Box::new(self.term.subst_type(v, ty)),
            handler: Box::new(self.handler.subst_type(v, ty)),
        }
        .into()
    }
}

impl<T> fmt::Display for TryWithVal<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "try {{ {} }} catch {{ {} }}", self.term, self.handler)
    }
}

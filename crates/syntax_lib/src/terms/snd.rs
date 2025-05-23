use super::Term;
use crate::{
    subst::{SubstTerm, SubstType},
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Snd<T>
where
    T: Term,
{
    term: Box<T>,
}

impl<T> Snd<T>
where
    T: Term,
{
    pub fn new<T1>(t: T1) -> Snd<T>
    where
        T1: Into<T>,
    {
        Snd {
            term: Box::new(t.into()),
        }
    }
}

impl<T> Term for Snd<T> where T: Term {}

impl<T> SubstTerm<T> for Snd<T>
where
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Snd {
            term: Box::new(self.term.subst(v, t)),
        }
        .into()
    }
}

impl<T> SubstType<<T as Term>::Type> for Snd<T>
where
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as Term>::Type) -> Self::Target {
        Snd {
            term: Box::new(self.term.subst_type(v, ty)),
        }
        .into()
    }
}

impl<T> fmt::Display for Snd<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}).1", self.term)
    }
}

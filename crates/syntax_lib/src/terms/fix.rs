use super::Term;
use crate::{
    TypeVar, Var,
    subst::{SubstTerm, SubstType},
    types::Type,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fix<T>
where
    T: Term,
{
    pub term: Box<T>,
}

impl<T> Fix<T>
where
    T: Term,
{
    pub fn new<T1>(t: T1) -> Fix<T>
    where
        T1: Into<T>,
    {
        Fix {
            term: Box::new(t.into()),
        }
    }
}

impl<T> Term for Fix<T> where T: Term {}

impl<T> SubstTerm<T> for Fix<T>
where
    T: Term + SubstTerm<T, Target = T>,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Fix {
            term: Box::new(self.term.subst(v, t)),
        }
        .into()
    }
}

impl<T, Ty> SubstType<Ty> for Fix<T>
where
    T: Term + SubstType<Ty, Target = T>,
    Ty: Type,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Fix {
            term: Box::new(self.term.subst_type(v, ty)),
        }
        .into()
    }
}

impl<T> fmt::Display for Fix<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fix({})", self.term)
    }
}

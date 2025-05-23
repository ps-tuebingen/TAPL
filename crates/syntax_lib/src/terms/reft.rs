use super::Term;
use crate::{
    subst::{SubstTerm, SubstType},
    types::Reference,
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ref<T>
where
    T: Term,
{
    term: Box<T>,
}

impl<T> Ref<T>
where
    T: Term,
{
    pub fn new<T1>(t: T1) -> Ref<T>
    where
        T1: Into<T>,
    {
        Ref {
            term: Box::new(t.into()),
        }
    }
}

impl<T> Term for Ref<T> where T: Term {}

impl<T> SubstTerm<T> for Ref<T>
where
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Ref {
            term: Box::new(self.term.subst(v, t)),
        }
        .into()
    }
}

impl<T> SubstType<<T as Term>::Type> for Ref<T>
where
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as Term>::Type) -> Self::Target {
        Ref {
            term: Box::new(self.term.subst_type(v, ty)),
        }
        .into()
    }
}

impl<T> fmt::Display for Ref<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ref({})", self.term)
    }
}
